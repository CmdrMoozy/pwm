use crypto::configuration::{Configuration, ConfigurationInstance};
use crypto::key::NormalKey;
use crypto::keystore::KeyStore;
use crypto::padding;
use error::Result;
use git2;
use repository::path::Path as RepositoryPath;
use sodiumoxide::crypto::secretbox;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use util::data::SensitiveData;
use util::git;
use util::lazy::Lazy;
use util::password_prompt;

lazy_static! {
    static ref CRYPTO_CONFIGURATION_PATH: PathBuf = PathBuf::from("crypto_configuration.mp");
    static ref KEYSTORE_PATH: PathBuf = PathBuf::from("keys.mp");
}

static MASTER_PASSWORD_PROMPT: &'static str = "Master password: ";
static ADD_KEY_PROMPT: &'static str = "Master password to add: ";
static REMOVE_KEY_PROMPT: &'static str = "Master password to remove: ";

static CRYPTO_CONFIGURATION_UPDATE_MESSAGE: &'static str = "Update encryption header contents.";
static KEYSTORE_UPDATE_MESSAGE: &'static str = "Update keys.";
static STORED_PASSWORD_UPDATE_MESSAGE: &'static str = "Update stored password / key.";
static STORED_PASSWORD_REMOVE_MESSAGE: &'static str = "Remove stored password / key.";

fn unwrap_password_or_prompt(password: Option<SensitiveData>,
                             prompt: &str,
                             confirm: bool)
                             -> Result<SensitiveData> {
    Ok(if let Some(p) = password {
        p
    } else {
        try!(password_prompt(prompt, confirm))
    })
}

fn get_commit_signature(repository: &git2::Repository) -> git2::Signature<'static> {
    repository.signature()
        .unwrap_or_else(|_| git2::Signature::now("pwm", "pwm@nowhere.com").unwrap())
}

fn open_crypto_configuration(repository: &git2::Repository) -> Result<ConfigurationInstance> {
    let mut path = PathBuf::from(try!(git::get_repository_workdir(repository)));
    path.push(CRYPTO_CONFIGURATION_PATH.as_path());
    ConfigurationInstance::new(path.as_path())
}

fn open_keystore(repository: &git2::Repository, key: &NormalKey) -> Result<KeyStore> {
    let mut path = PathBuf::from(try!(git::get_repository_workdir(repository)));
    path.push(KEYSTORE_PATH.as_path());
    KeyStore::open_or_new(path.as_path(), &key)
}

fn write_encrypt(path: &RepositoryPath,
                 plaintext: SensitiveData,
                 master_key: &NormalKey)
                 -> Result<()> {
    let (nonce, data) = master_key.encrypt(padding::pad(plaintext));

    if let Some(parent) = path.absolute_path().parent() {
        try!(fs::create_dir_all(parent));
    }

    use std::io::Write;
    let mut file = try!(File::create(path.absolute_path()));
    try!(file.write_all(&nonce.0));
    try!(file.write_all(data.as_slice()));
    try!(file.flush());
    Ok(())
}

fn read_decrypt(path: &RepositoryPath, master_key: &NormalKey) -> Result<SensitiveData> {
    use std::io::Read;

    if !path.absolute_path().exists() {
        bail!("No stored password at path '{}'",
              path.relative_path().display());
    }

    let mut file = try!(File::open(path.absolute_path()));
    let mut nonce: secretbox::Nonce = secretbox::Nonce([0; 24]);
    let mut data: Vec<u8> = vec![];
    try!(file.read_exact(&mut nonce.0));
    try!(file.read_to_end(&mut data));

    padding::unpad(try!(master_key.decrypt(data.as_slice(), &nonce)))
}

pub struct Repository {
    repository: git2::Repository,
    // NOTE: crypto_configuration is guaranteed to be Some() everywhere except within drop().
    crypto_configuration: Option<ConfigurationInstance>,
    // NOTE: keystore is guaranteed to be Some() everywhere except within drop().
    keystore: Option<Lazy<'static, Result<KeyStore>>>,
}

impl Repository {
    /// Construct a new Repository handle. If the repository doesn't exist, and
    /// create = true, then a new repository will be initialized. If no master
    /// password is provided, we will prompt for one on stdin.
    pub fn new<P: AsRef<Path>>(path: P,
                               create: bool,
                               password: Option<SensitiveData>)
                               -> Result<Repository> {
        let repository = try!(git::open_repository(path.as_ref(), create));
        let crypto_configuration = try!(open_crypto_configuration(&repository));

        let c = crypto_configuration.get();
        let path = path.as_ref().to_path_buf();
        let keystore = Lazy::new(move || -> Result<KeyStore> {
            let master_password =
                try!(unwrap_password_or_prompt(password, MASTER_PASSWORD_PROMPT, create));
            let master_key = try!(NormalKey::new_password(master_password, Some(c)));
            let repository = try!(git::open_repository(path.as_path(), false));
            open_keystore(&repository, &master_key)
        });

        Ok(Repository {
            repository: repository,
            crypto_configuration: Some(crypto_configuration),
            keystore: Some(keystore),
        })
    }

    pub fn path<P: AsRef<Path>>(&self, path: P) -> Result<RepositoryPath> {
        RepositoryPath::new(try!(self.workdir()), path)
    }

    pub fn get_crypto_configuration(&self) -> Configuration {
        self.crypto_configuration.as_ref().unwrap().get()
    }

    fn get_key_store(&self) -> Result<&KeyStore> {
        match self.keystore.as_ref().unwrap().as_ref() {
            Ok(ks) => Ok(ks),
            Err(e) => bail!("Accessing repository key store failed: {}", e),
        }
    }

    fn get_key_store_mut(&mut self) -> Result<&mut KeyStore> {
        use std::ops::DerefMut;
        let lazy: &mut Lazy<'static, Result<KeyStore>> = self.keystore.as_mut().unwrap();
        lazy.deref_mut()
            .as_mut()
            .map_err(|e| format!("Accessing repository key store failed: {}", e).into())
    }

    fn get_master_key(&self) -> Result<&NormalKey> { Ok(try!(self.get_key_store()).get_key()) }

    pub fn workdir(&self) -> Result<&Path> { git::get_repository_workdir(&self.repository) }

    fn commit_all(&self, message: &str, paths: &[&Path]) -> Result<()> {
        try!(git::commit_paths(&self.repository,
                               Some(&get_commit_signature(&self.repository)),
                               Some(&get_commit_signature(&self.repository)),
                               message,
                               paths));
        Ok(())
    }

    fn commit_one(&self, message: &str, path: &Path) -> Result<()> {
        self.commit_all(message, &[path])
    }

    pub fn list(&self, path_filter: Option<&RepositoryPath>) -> Result<Vec<RepositoryPath>> {
        let default_path_filter = try!(self.path(""));
        let path_filter: &RepositoryPath = path_filter.unwrap_or(&default_path_filter);
        let entries = try!(git::get_repository_listing(&self.repository,
                                                       path_filter.relative_path()));
        entries.into_iter()
            .filter(|entry| entry != CRYPTO_CONFIGURATION_PATH.as_path())
            .filter(|entry| entry != KEYSTORE_PATH.as_path())
            .map(|entry| self.path(entry))
            .collect()
    }

    pub fn add_key(&mut self, password: Option<SensitiveData>) -> Result<()> {
        let config = self.get_crypto_configuration();
        let password = try!(unwrap_password_or_prompt(password, ADD_KEY_PROMPT, true));
        let key = try!(NormalKey::new_password(password, Some(config)));
        let was_added = try!(try!(self.get_key_store_mut()).add(&key));
        if !was_added {
            bail!("The specified key is already in use, so it was not re-added");
        }
        Ok(())
    }

    pub fn remove_key(&mut self, password: Option<SensitiveData>) -> Result<()> {
        let config = self.get_crypto_configuration();
        let password = try!(unwrap_password_or_prompt(password, REMOVE_KEY_PROMPT, true));
        let key = try!(NormalKey::new_password(password, Some(config)));
        let was_removed = try!(try!(self.get_key_store_mut()).remove(&key));
        if !was_removed {
            bail!("The specified key is not registered with this repository");
        }
        Ok(())
    }

    pub fn write_encrypt(&self, path: &RepositoryPath, plaintext: SensitiveData) -> Result<()> {
        try!(write_encrypt(path, plaintext, try!(self.get_master_key())));
        try!(self.commit_one(STORED_PASSWORD_UPDATE_MESSAGE, path.relative_path()));
        Ok(())
    }

    pub fn read_decrypt(&self, path: &RepositoryPath) -> Result<SensitiveData> {
        read_decrypt(path, try!(self.get_master_key()))
    }

    pub fn remove(&self, path: &RepositoryPath) -> Result<()> {
        try!(fs::remove_file(path.absolute_path()));
        try!(self.commit_one(STORED_PASSWORD_REMOVE_MESSAGE, path.relative_path()));
        Ok(())
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        self.keystore.take();
        self.commit_one(KEYSTORE_UPDATE_MESSAGE, KEYSTORE_PATH.as_path()).unwrap();

        self.crypto_configuration.take().unwrap().close().unwrap();
        self.commit_one(CRYPTO_CONFIGURATION_UPDATE_MESSAGE,
                        CRYPTO_CONFIGURATION_PATH.as_path())
            .unwrap();
    }
}
