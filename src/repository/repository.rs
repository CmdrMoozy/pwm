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

use ::crypto::decrypt::decrypt;
use ::crypto::encrypt::encrypt;
use ::crypto::key::Key;
use ::crypto::padding;
use ::error::{Error, ErrorKind, Result};
use git2;
use ::repository::{CryptoConfiguration, CryptoConfigurationInstance};
use ::repository::Path as RepositoryPath;
use sodiumoxide::crypto::secretbox;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use ::util::data::SensitiveData;
use ::util::git;

lazy_static! {
    static ref CRYPTO_CONFIGURATION_PATH: PathBuf = PathBuf::from("crypto_configuration.mp");
}

static CRYPTO_CONFIGURATION_UPDATE_MESSAGE: &'static str = "Update encryption header contents.";
static STORED_PASSWORD_UPDATE_MESSAGE: &'static str = "Update stored password / key.";

pub struct Repository {
    repository: git2::Repository,
    crypto_configuration: Option<CryptoConfigurationInstance>,
}

impl Repository {
    pub fn new<P: AsRef<Path>>(path: P, create: bool) -> Result<Repository> {
        let repository = try!(git::open_repository(path, create));

        let mut crypto_configuration_path = PathBuf::from(repository.workdir().unwrap());
        crypto_configuration_path.push(CRYPTO_CONFIGURATION_PATH.as_path());

        Ok(Repository {
            repository: repository,
            crypto_configuration:
                Some(try!(CryptoConfigurationInstance::new(crypto_configuration_path.as_path()))),
        })
    }

    pub fn get_crypto_configuration(&self) -> Result<CryptoConfiguration> {
        self.crypto_configuration.as_ref().unwrap().get()
    }

    pub fn set_crypto_configuration(&self, config: CryptoConfiguration) -> Result<()> {
        self.crypto_configuration.as_ref().unwrap().set(config)
        // TODO: Persist config, update all encrypted data to match, commit the result.
    }

    pub fn reset_crypto_configuration(&self) -> Result<()> {
        self.crypto_configuration.as_ref().unwrap().reset()
        // TODO: Persist config, update all encrypted data to match, commit the result.
    }

    pub fn workdir(&self) -> Result<&Path> {
        match self.repository.workdir() {
            Some(path) => Ok(path),
            None => {
                Err(Error::new(ErrorKind::Repository {
                    description: "Repository has no workdir".to_owned(),
                }))
            },
        }
    }

    fn build_key(&self, password: SensitiveData) -> Result<Key> {
        let config = try!(self.get_crypto_configuration());
        Key::new(password,
                 Some(config.get_salt()),
                 Some(config.get_ops_limit()),
                 Some(config.get_mem_limit()))
    }

    pub fn list(&self, path_filter: &RepositoryPath) -> Result<Vec<PathBuf>> {
        let entries = try!(git::get_repository_listing(&self.repository,
                                                       path_filter.relative_path()));
        Ok(entries.into_iter()
            .filter(|entry| entry != CRYPTO_CONFIGURATION_PATH.as_path())
            .collect())
    }

    pub fn write_encrypt(&self,
                         path: &RepositoryPath,
                         plaintext: SensitiveData,
                         key_password: SensitiveData)
                         -> Result<()> {
        let key = try!(self.build_key(key_password));
        let (nonce, data) = try!(encrypt(padding::pad(plaintext), &key));

        if let Some(parent) = path.absolute_path().parent() {
            try!(fs::create_dir_all(parent));
        }

        {
            use std::io::Write;
            let mut file = try!(File::create(path.absolute_path()));
            try!(file.write_all(&nonce.0));
            try!(file.write_all(data.as_slice()));
            try!(file.flush());
        }

        try!(git::commit_paths(&self.repository,
                               None,
                               None,
                               STORED_PASSWORD_UPDATE_MESSAGE,
                               &[PathBuf::from(path.relative_path()).as_path()]));
        Ok(())
    }

    pub fn read_decrypt(&self,
                        path: &RepositoryPath,
                        key_password: SensitiveData)
                        -> Result<SensitiveData> {
        use std::io::Read;

        let key = try!(self.build_key(key_password));

        let mut file = try!(File::open(path.absolute_path()));
        let mut nonce: secretbox::Nonce = secretbox::Nonce([0; 24]);
        let mut data: Vec<u8> = vec![];
        try!(file.read_exact(&mut nonce.0));
        try!(file.read_to_end(&mut data));

        padding::unpad(try!(decrypt(data.as_slice(), &nonce, &key)))
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        self.crypto_configuration.take().unwrap().close().unwrap();
        git::commit_paths(&self.repository,
                          None,
                          None,
                          CRYPTO_CONFIGURATION_UPDATE_MESSAGE,
                          &[CRYPTO_CONFIGURATION_PATH.as_path()])
            .unwrap();
    }
}
