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
use crate::repository::keystore::{
    add_key, add_password_key, get_keystore, remove_key, remove_password_key,
};
use crate::repository::path::Path as RepositoryPath;
use crate::util::git;
use crate::util::lazy::{new_lazy_result, LazyResult};
use bdrck::crypto::key::{AbstractKey, Key, Nonce};
use bdrck::crypto::keystore::DiskKeyStore;
use bdrck::crypto::secret::Secret;
use git2;
use once_cell::sync::Lazy;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

static CRYPTO_CONFIGURATION_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("crypto_configuration.mp"));
static KEYSTORE_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("keys.mp"));

static CRYPTO_CONFIGURATION_UPDATE_MESSAGE: &'static str = "Update encryption header contents.";
static KEYSTORE_UPDATE_MESSAGE: &'static str = "Update keys.";
static STORED_PASSWORD_UPDATE_MESSAGE: &'static str = "Update stored password / key.";
static STORED_PASSWORD_REMOVE_MESSAGE: &'static str = "Remove stored password / key.";

fn get_keystore_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let repository = git::open_repository(path.as_ref(), /*create=*/ false)?;
    let mut path = PathBuf::from(git::get_repository_workdir(&repository)?);
    path.push(KEYSTORE_PATH.as_path());
    Ok(path)
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

fn write_encrypt(
    path: &RepositoryPath,
    mut plaintext: Secret,
    master_key: &Key,
    nonce: Option<Nonce>,
) -> Result<()> {
    padding::pad(&mut plaintext)?;
    let encrypted_tuple = master_key.encrypt(&plaintext, nonce)?;

    if let Some(parent) = path.absolute_path().parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path.absolute_path())?;
    rmp_serde::encode::write(&mut file, &encrypted_tuple)?;
    file.flush()?;
    Ok(())
}

pub struct Repository {
    repository: git2::Repository,
    // NOTE: crypto_configuration is guaranteed to be Some() everywhere except within drop().
    crypto_configuration: Option<ConfigurationInstance>,
    // NOTE: keystore is guaranteed to be Some() everywhere except within drop().
    keystore: Option<LazyResult<DiskKeyStore, Error>>,
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
        let keystore_path = get_keystore_path(path)?;
        let keystore_is_new = !keystore_path.exists();
        let keystore = new_lazy_result(move || get_keystore(&keystore_path, create, &c, password));

        // If we're initializing a brand new key store, `force` it so we add an
        // initial master key.
        if keystore_is_new {
            use std::ops::Deref;

            keystore.deref().get()?;
        }

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

    pub fn set_crypto_configuration(&self, crypto_configuration: Configuration) {
        self.crypto_configuration
            .as_ref()
            .unwrap()
            .set(crypto_configuration);
    }

    fn get_key_store(&self) -> Result<&DiskKeyStore> {
        use std::ops::Deref;

        self.keystore.as_ref().unwrap().deref().get()
    }

    fn get_key_store_mut(&mut self) -> Result<&mut DiskKeyStore> {
        use std::ops::DerefMut;

        self.keystore.as_mut().unwrap().deref_mut().get_mut()
    }

    fn get_master_key(&self) -> Result<&Key> {
        Ok(self.get_key_store()?.get_master_key()?)
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

    pub fn add_key<E: Into<Error>, K: AbstractKey<Error = E>>(&mut self, key: &K) -> Result<()> {
        add_key(self.get_key_store_mut()?, key)
    }

    pub fn add_password_key(&mut self, password: Option<Secret>) -> Result<()> {
        add_password_key(
            &self.get_crypto_configuration(),
            self.get_key_store_mut()?,
            password,
        )
    }

    pub fn remove_key<E: Into<Error>, K: AbstractKey<Error = E>>(&mut self, key: &K) -> Result<()> {
        remove_key(self.get_key_store_mut()?, key)
    }

    pub fn remove_password_key(&mut self, password: Option<Secret>) -> Result<()> {
        remove_password_key(
            &self.get_crypto_configuration(),
            self.get_key_store_mut()?,
            password,
        )
    }

    pub fn write_encrypt(
        &mut self,
        path: &RepositoryPath,
        plaintext: Secret,
        nonce: Option<Nonce>,
    ) -> Result<()> {
        write_encrypt(path, plaintext, self.get_master_key()?, nonce)?;
        self.commit_one(STORED_PASSWORD_UPDATE_MESSAGE, path.relative_path())?;
        Ok(())
    }

    pub fn read_decrypt(&self, path: &RepositoryPath) -> Result<Secret> {
        if !path.absolute_path().exists() {
            bail!(
                "no stored password at path '{}'",
                path.relative_path().display()
            );
        }

        let mut file = File::open(path.absolute_path())?;
        let encrypted_tuple: (Option<Nonce>, Vec<u8>) = rmp_serde::decode::from_read(&mut file)?;
        let mut decrypted: Secret = self
            .get_master_key()?
            .decrypt(encrypted_tuple.0.as_ref(), encrypted_tuple.1.as_slice())?
            .into();
        padding::unpad(&mut decrypted)?;

        Ok(decrypted)
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
