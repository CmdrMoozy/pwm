// pwm - A simple password manager for Linux.
// Copyright (C) 2015  Axel Rasmussen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crypto::key::NormalKey;
use crypto::keystore::KeyStore;
use crypto::padding;
use error::Result;
use git2;
use repository::configuration::{Configuration, ConfigurationInstance};
use repository::path::Path as RepositoryPath;
use sodiumoxide::crypto::secretbox;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use util::data::SensitiveData;
use util::git;
use util::password_prompt;

lazy_static! {
    static ref CRYPTO_CONFIGURATION_PATH: PathBuf = PathBuf::from("crypto_configuration.mp");
    static ref KEYSTORE_PATH: PathBuf = PathBuf::from("keys.mp");
}

static MASTER_PASSWORD_PROMPT: &'static str = "Master password: ";

static CRYPTO_CONFIGURATION_UPDATE_MESSAGE: &'static str = "Update encryption header contents.";
static KEYSTORE_UPDATE_MESSAGE: &'static str = "Update keys.";
static STORED_PASSWORD_UPDATE_MESSAGE: &'static str = "Update stored password / key.";
static STORED_PASSWORD_REMOVE_MESSAGE: &'static str = "Remove stored password / key.";

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
    keystore: Option<KeyStore>,
}

impl Repository {
    /// Construct a new Repository handle. If the repository doesn't exist, and
    /// create = true, then a new repository will be initialized. If no master
    /// password is provided, we will prompt for one on stdin.
    pub fn new<P: AsRef<Path>>(path: P,
                               create: bool,
                               password: Option<SensitiveData>)
                               -> Result<Repository> {
        let repository = try!(git::open_repository(path, create));
        let crypto_configuration = try!(open_crypto_configuration(&repository));
        let master_password: SensitiveData = if let Some(p) = password {
            p
        } else {
            try!(password_prompt(MASTER_PASSWORD_PROMPT, create))
        };
        let master_key = try!(crypto_configuration.get().build_key(master_password));
        let keystore = try!(open_keystore(&repository, &master_key));

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

    fn get_master_key(&self) -> &NormalKey { self.keystore.as_ref().unwrap().get_key() }

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

    pub fn write_encrypt(&self, path: &RepositoryPath, plaintext: SensitiveData) -> Result<()> {
        try!(write_encrypt(path, plaintext, self.get_master_key()));
        try!(self.commit_one(STORED_PASSWORD_UPDATE_MESSAGE, path.relative_path()));
        Ok(())
    }

    pub fn read_decrypt(&self, path: &RepositoryPath) -> Result<SensitiveData> {
        read_decrypt(path, self.get_master_key())
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
