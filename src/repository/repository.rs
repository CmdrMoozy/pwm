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

use crypto::decrypt::decrypt;
use crypto::encrypt::encrypt;
use crypto::key::Key;
use crypto::padding;
use error::{Error, ErrorKind, Result};
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
    static ref AUTH_TOKEN_PATH: PathBuf = PathBuf::from(".auth_token");

    static ref AUTH_TOKEN_CONTENTS: Vec<u8> =
        "3c017f717b39247c351154a41d2850e4187284da4b928f13c723d54440ba2dfe".bytes().collect();
}

static MASTER_PASSWORD_PROMPT: &'static str = "Master password: ";

static CRYPTO_CONFIGURATION_UPDATE_MESSAGE: &'static str = "Update encryption header contents.";
static STORED_PASSWORD_UPDATE_MESSAGE: &'static str = "Update stored password / key.";
static STORED_PASSWORD_REMOVE_MESSAGE: &'static str = "Remove stored password / key.";

fn get_commit_signature(repository: &git2::Repository) -> git2::Signature<'static> {
    repository.signature().unwrap_or(git2::Signature::now("pwm", "pwm@nowhere.com").unwrap())
}

fn open_crypto_configuration(repository: &git2::Repository) -> Result<ConfigurationInstance> {
    let mut path = PathBuf::from(try!(git::get_repository_workdir(repository)));
    path.push(CRYPTO_CONFIGURATION_PATH.as_path());
    ConfigurationInstance::new(path.as_path())
}

fn write_encrypt(repository: &git2::Repository,
                 path: &RepositoryPath,
                 plaintext: SensitiveData,
                 master_key: &Key)
                 -> Result<()> {
    let (nonce, data) = try!(encrypt(padding::pad(plaintext), &master_key));

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

    try!(git::commit_paths(&repository,
                           Some(&get_commit_signature(repository)),
                           Some(&get_commit_signature(repository)),
                           STORED_PASSWORD_UPDATE_MESSAGE,
                           &[PathBuf::from(path.relative_path()).as_path()]));
    Ok(())
}

fn read_decrypt(path: &RepositoryPath, master_key: &Key) -> Result<SensitiveData> {
    use std::io::Read;

    let mut file = try!(File::open(path.absolute_path()));
    let mut nonce: secretbox::Nonce = secretbox::Nonce([0; 24]);
    let mut data: Vec<u8> = vec![];
    try!(file.read_exact(&mut nonce.0));
    try!(file.read_to_end(&mut data));

    padding::unpad(try!(decrypt(data.as_slice(), &nonce, &master_key)))
}

fn write_auth_token(repository: &git2::Repository, master_key: &Key) -> Result<()> {
    write_encrypt(repository,
                  &try!(RepositoryPath::new(try!(git::get_repository_workdir(repository)),
                                            AUTH_TOKEN_PATH.as_path())),
                  SensitiveData::from(AUTH_TOKEN_CONTENTS.clone()),
                  master_key)
}

fn verify_auth_token(repository: &git2::Repository, create: bool, master_key: &Key) -> Result<()> {
    let mut path = PathBuf::from(try!(git::get_repository_workdir(repository)));
    path.push(AUTH_TOKEN_PATH.as_path());

    if create && !path.exists() {
        try!(write_auth_token(repository, master_key));
    }

    if let Ok(token) =
        read_decrypt(&try!(RepositoryPath::new(try!(git::get_repository_workdir(repository)),
                                               AUTH_TOKEN_PATH.as_path())),
                     master_key) {
        if &token[..] == &AUTH_TOKEN_CONTENTS[..] {
            return Ok(());
        }
    }

    Err(Error::new(ErrorKind::Crypto { cause: "Incorrect master password".to_owned() }))
}

fn reencrypt_all(repository: &git2::Repository,
                 listing: &Vec<RepositoryPath>,
                 old_master_key: &Key,
                 new_master_key: &Key)
                 -> Result<()> {
    for path in listing {
        try!(write_encrypt(repository,
                           path,
                           try!(read_decrypt(path, old_master_key)),
                           new_master_key));
    }
    try!(write_auth_token(repository, new_master_key));
    Ok(())
}

pub struct Repository {
    repository: git2::Repository,
    // NOTE: crypto_configuration is guaranteed to be Some() everywhere except within drop().
    crypto_configuration: Option<ConfigurationInstance>,
    master_password: SensitiveData,
    master_key: Key,
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
        let master_key = try!(crypto_configuration.get().build_key(master_password.clone()));

        try!(verify_auth_token(&repository, create, &master_key));

        Ok(Repository {
            repository: repository,
            crypto_configuration: Some(crypto_configuration),
            master_password: master_password,
            master_key: master_key,
        })
    }

    pub fn path<P: AsRef<Path>>(&self, path: P) -> Result<RepositoryPath> {
        RepositoryPath::new(try!(self.workdir()), path)
    }

    pub fn get_crypto_configuration(&self) -> Configuration {
        self.crypto_configuration.as_ref().unwrap().get()
    }

    pub fn set_crypto_configuration(&mut self, config: Configuration) -> Result<()> {
        self.crypto_configuration.as_ref().unwrap().set(config.clone());
        let master_password = self.master_password.clone();
        self.reencrypt(try!(config.build_key(master_password)))
    }

    pub fn reset_crypto_configuration(&mut self) -> Result<()> {
        self.crypto_configuration.as_ref().unwrap().reset();
        let crypto_configuration = self.get_crypto_configuration();
        let master_password = self.master_password.clone();
        self.reencrypt(try!(crypto_configuration.build_key(master_password)))
    }

    pub fn workdir(&self) -> Result<&Path> { git::get_repository_workdir(&self.repository) }

    pub fn list(&self, path_filter: Option<&RepositoryPath>) -> Result<Vec<RepositoryPath>> {
        let default_path_filter = try!(self.path(""));
        let path_filter: &RepositoryPath = path_filter.unwrap_or(&default_path_filter);
        let entries = try!(git::get_repository_listing(&self.repository,
                                                       path_filter.relative_path()));
        let entries: Result<Vec<RepositoryPath>> = entries.into_iter()
            .filter(|entry| entry != CRYPTO_CONFIGURATION_PATH.as_path())
            .filter(|entry| entry != AUTH_TOKEN_PATH.as_path())
            .map(|entry| self.path(entry))
            .collect();
        entries
    }

    pub fn write_encrypt(&self, path: &RepositoryPath, plaintext: SensitiveData) -> Result<()> {
        write_encrypt(&self.repository, path, plaintext, &self.master_key)
    }

    pub fn read_decrypt(&self, path: &RepositoryPath) -> Result<SensitiveData> {
        read_decrypt(path, &self.master_key)
    }

    pub fn remove(&self, path: &RepositoryPath) -> Result<()> {
        try!(fs::remove_file(path.absolute_path()));
        try!(git::commit_paths(&self.repository,
                               Some(&get_commit_signature(&self.repository)),
                               Some(&get_commit_signature(&self.repository)),
                               STORED_PASSWORD_REMOVE_MESSAGE,
                               &[path.relative_path()]));
        Ok(())
    }

    fn reencrypt(&mut self, new_master_key: Key) -> Result<()> {
        {
            let old_master_key = &self.master_key;
            try!(reencrypt_all(&self.repository,
                               &try!(self.list(None)),
                               old_master_key,
                               &new_master_key));
        }

        self.master_key = new_master_key;
        Ok(())
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        self.crypto_configuration.take().unwrap().close().unwrap();
        git::commit_paths(&self.repository,
                          Some(&get_commit_signature(&self.repository)),
                          Some(&get_commit_signature(&self.repository)),
                          CRYPTO_CONFIGURATION_UPDATE_MESSAGE,
                          &[CRYPTO_CONFIGURATION_PATH.as_path()])
            .unwrap();
    }
}
