// Copyright 2015 Axel Rasmussen
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::crypto::configuration::{Configuration, ConfigurationInstance};
use crate::crypto::padding;
use crate::error::*;
use crate::repository::path::Path as RepositoryPath;
use crate::util::data::Secret;
use crate::util::git;
use crate::util::lazy::Lazy;
use crate::util::password_prompt;
use bdrck::crypto::key::{AbstractKey, Key, Nonce};
use bdrck::crypto::keystore::DiskKeyStore;
use failure::format_err;
use git2;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use yubirs::piv;
use yubirs::piv::hal::PcscHardware;

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

fn unwrap_password_or_prompt(
    password: Option<Secret>,
    prompt: &str,
    confirm: bool,
) -> Result<Secret> {
    Ok(if let Some(p) = password {
        p
    } else {
        password_prompt(prompt, confirm)?
    })
}

fn get_keystore_key_password(
    create: bool,
    password: Option<Secret>,
    crypto_config: &Configuration,
) -> Result<Box<dyn AbstractKey>> {
    let password = unwrap_password_or_prompt(password, MASTER_PASSWORD_PROMPT, create)?;
    Ok(Box::new(Key::new_password(
        password.as_slice(),
        &crypto_config.get_salt(),
        crypto_config.get_ops_limit(),
        crypto_config.get_mem_limit(),
    )?))
}

#[cfg(feature = "piv")]
fn get_keystore_key(
    create: bool,
    password: Option<Secret>,
    crypto_config: &Configuration,
) -> Result<Box<dyn AbstractKey>> {
    // Don't try using a PIV device if a password was given explicitly.
    if password.is_none() {
        let config = crate::configuration::get()?;
        if let Some(config) = config.piv {
            let handle: piv::Handle<PcscHardware> = piv::Handle::new()?;
            let readers: HashSet<String> = HashSet::from_iter(handle.list_readers()?.into_iter());

            for config in config.keys {
                let reader: String = config
                    .reader
                    .unwrap_or_else(|| piv::DEFAULT_READER.to_owned());
                if readers.contains(&reader) {
                    let key: piv::key::Key<PcscHardware> = piv::key::Key::new(
                        Some(reader.as_str()),
                        None,
                        config.slot,
                        config.public_key.as_path(),
                    )?;
                    return Ok(Box::new(key));
                }
            }
        }
    }

    get_keystore_key_password(create, password, crypto_config)
}

#[cfg(not(feature = "piv"))]
fn get_keystore_key(
    create: bool,
    password: Option<Secret>,
    crypto_config: &Configuration,
) -> Result<Box<AbstractKey>> {
    get_keystore_key_password(create, password, crypto_config)
}

fn open_keystore(
    path: PathBuf,
    create: bool,
    password: Option<Secret>,
    crypto_config: Configuration,
) -> Result<DiskKeyStore> {
    let repository = git::open_repository(path.as_path(), false)?;
    let mut path = PathBuf::from(git::get_repository_workdir(&repository)?);
    path.push(KEYSTORE_PATH.as_path());

    let key = get_keystore_key(create, password, &crypto_config)?;

    Ok(DiskKeyStore::open_or_new(path.as_path(), &key)?)
}

fn get_commit_signature(repository: &git2::Repository) -> git2::Signature<'static> {
    repository
        .signature()
        .unwrap_or_else(|_| git2::Signature::now("pwm", "pwm@nowhere.com").unwrap())
}

fn open_crypto_configuration(repository: &git2::Repository) -> Result<ConfigurationInstance> {
    let mut path = PathBuf::from(git::get_repository_workdir(repository)?);
    path.push(CRYPTO_CONFIGURATION_PATH.as_path());
    ConfigurationInstance::new(path.as_path())
}

fn write_encrypt(path: &RepositoryPath, mut plaintext: Secret, master_key: &Key) -> Result<()> {
    padding::pad(&mut plaintext);
    let encrypted_tuple: (Option<Nonce>, Secret) =
        master_key.encrypt(plaintext.as_slice(), None)?;

    if let Some(parent) = path.absolute_path().parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path.absolute_path())?;
    rmp_serde::encode::write(&mut file, &encrypted_tuple)?;
    file.flush()?;
    Ok(())
}

fn read_decrypt(path: &RepositoryPath, master_key: &Key) -> Result<Secret> {
    if !path.absolute_path().exists() {
        return Err(Error::NotFound(format_err!(
            "No stored password at path '{}'",
            path.relative_path().display()
        )));
    }

    let mut file = File::open(path.absolute_path())?;
    let encrypted_tuple: (Option<Nonce>, Secret) = rmp_serde::decode::from_read(&mut file)?;
    let mut decrypted: Secret =
        master_key.decrypt(encrypted_tuple.0.as_ref(), encrypted_tuple.1.as_slice())?;
    padding::unpad(&mut decrypted)?;
    Ok(decrypted)
}

pub struct Repository {
    repository: git2::Repository,
    // NOTE: crypto_configuration is guaranteed to be Some() everywhere except within drop().
    crypto_configuration: Option<ConfigurationInstance>,
    // NOTE: keystore is guaranteed to be Some() everywhere except within drop().
    keystore: Option<Lazy<'static, Result<DiskKeyStore>>>,
}

impl Repository {
    /// Construct a new Repository handle. If the repository doesn't exist, and
    /// create = true, then a new repository will be initialized. If no master
    /// password is provided, we will prompt for one on stdin.
    pub fn new<P: AsRef<Path>>(
        path: P,
        create: bool,
        password: Option<Secret>,
    ) -> Result<Repository> {
        let repository = git::open_repository(path.as_ref(), create)?;
        let crypto_configuration = open_crypto_configuration(&repository)?;

        let c = crypto_configuration.get();
        let path = path.as_ref().to_path_buf();
        let keystore = Lazy::new(move || open_keystore(path, create, password, c));

        Ok(Repository {
            repository: repository,
            crypto_configuration: Some(crypto_configuration),
            keystore: Some(keystore),
        })
    }

    pub fn path<P: AsRef<Path>>(&self, path: P) -> Result<RepositoryPath> {
        RepositoryPath::new(self.workdir()?, path)
    }

    pub fn get_crypto_configuration(&self) -> Configuration {
        self.crypto_configuration.as_ref().unwrap().get()
    }

    fn get_key_store(&self) -> Result<&DiskKeyStore> {
        use std::ops::Deref;
        let lazy: &Lazy<'static, Result<DiskKeyStore>> = self.keystore.as_ref().unwrap();
        let result: &Result<DiskKeyStore> = lazy.deref();
        result.as_ref().map_err(|e| {
            Error::Internal(format_err!("Accessing repository key store failed: {}", e))
        })
    }

    fn get_key_store_mut(&mut self) -> Result<&mut DiskKeyStore> {
        use std::ops::DerefMut;
        let lazy: &mut Lazy<'static, Result<DiskKeyStore>> = self.keystore.as_mut().unwrap();
        let result: &mut Result<DiskKeyStore> = lazy.deref_mut();
        result.as_mut().map_err(|e| {
            Error::Internal(format_err!("Accessing repository key store failed: {}", e))
        })
    }

    fn get_master_key(&self) -> Result<&Key> {
        Ok(self.get_key_store()?.get_master_key())
    }

    pub fn workdir(&self) -> Result<&Path> {
        git::get_repository_workdir(&self.repository)
    }

    fn commit_all(&self, message: &str, paths: &[&Path]) -> Result<()> {
        git::commit_paths(
            &self.repository,
            Some(&get_commit_signature(&self.repository)),
            Some(&get_commit_signature(&self.repository)),
            message,
            paths,
        )?;
        Ok(())
    }

    fn commit_one(&self, message: &str, path: &Path) -> Result<()> {
        self.commit_all(message, &[path])
    }

    pub fn list(&self, path_filter: Option<&RepositoryPath>) -> Result<Vec<RepositoryPath>> {
        let default_path_filter = self.path("")?;
        let path_filter: &RepositoryPath = path_filter.unwrap_or(&default_path_filter);
        let entries = git::get_repository_listing(&self.repository, path_filter.relative_path())?;
        entries
            .into_iter()
            .filter(|entry| entry != CRYPTO_CONFIGURATION_PATH.as_path())
            .filter(|entry| entry != KEYSTORE_PATH.as_path())
            .map(|entry| self.path(entry))
            .collect()
    }

    pub fn add_key(&mut self, password: Option<Secret>) -> Result<()> {
        let config = self.get_crypto_configuration();
        let password = unwrap_password_or_prompt(password, ADD_KEY_PROMPT, true)?;
        let key = Key::new_password(
            password.as_slice(),
            &config.get_salt(),
            config.get_ops_limit(),
            config.get_mem_limit(),
        )?;
        let was_added = self.get_key_store_mut()?.add_key(&key)?;
        if !was_added {
            return Err(Error::InvalidArgument(format_err!(
                "The specified key is already in use, so it was not re-added"
            )));
        }
        Ok(())
    }

    pub fn remove_key(&mut self, password: Option<Secret>) -> Result<()> {
        let config = self.get_crypto_configuration();
        let password = unwrap_password_or_prompt(password, REMOVE_KEY_PROMPT, true)?;
        let key = Key::new_password(
            password.as_slice(),
            &config.get_salt(),
            config.get_ops_limit(),
            config.get_mem_limit(),
        )?;
        let was_removed = self.get_key_store_mut()?.remove_key(&key)?;
        if !was_removed {
            return Err(Error::NotFound(format_err!(
                "The specified key is not registered with this repository"
            )));
        }
        Ok(())
    }

    pub fn write_encrypt(&mut self, path: &RepositoryPath, plaintext: Secret) -> Result<()> {
        write_encrypt(path, plaintext, self.get_master_key()?)?;
        self.commit_one(STORED_PASSWORD_UPDATE_MESSAGE, path.relative_path())?;
        Ok(())
    }

    pub fn read_decrypt(&self, path: &RepositoryPath) -> Result<Secret> {
        read_decrypt(path, self.get_master_key()?)
    }

    pub fn remove(&mut self, path: &RepositoryPath) -> Result<()> {
        fs::remove_file(path.absolute_path())?;
        self.commit_one(STORED_PASSWORD_REMOVE_MESSAGE, path.relative_path())?;
        Ok(())
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        self.keystore.take();
        self.commit_one(KEYSTORE_UPDATE_MESSAGE, KEYSTORE_PATH.as_path())
            .unwrap();

        self.crypto_configuration.take().unwrap().close().unwrap();
        self.commit_one(
            CRYPTO_CONFIGURATION_UPDATE_MESSAGE,
            CRYPTO_CONFIGURATION_PATH.as_path(),
        )
        .unwrap();
    }
}
