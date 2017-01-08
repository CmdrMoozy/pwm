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

use ::error::Result;
use git2;
use ::repository::{CryptoConfiguration, CryptoConfigurationInstance};
use std::path::{Path, PathBuf};
use ::util::git;

static CRYPTO_CONFIGURATION_PATH: &'static str = "crypto_configuration.mp";
static CRYPTO_CONFIGURATION_UPDATE_MESSAGE: &'static str = "Update encryption header contents.";

pub struct Repository {
    repository: git2::Repository,
    crypto_configuration: Option<CryptoConfigurationInstance>,
}

impl Repository {
    pub fn new<P: AsRef<Path>>(path: P, create: bool) -> Result<Repository> {
        let repository = try!(git::open_repository(path, create));

        let mut crypto_configuration_path = PathBuf::from(repository.workdir().unwrap());
        crypto_configuration_path.push(CRYPTO_CONFIGURATION_PATH);

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

    pub fn workdir(&self) -> Option<&Path> { self.repository.workdir() }
}

impl Drop for Repository {
    fn drop(&mut self) {
        self.crypto_configuration.take().unwrap().close().unwrap();
        git::commit_paths(&self.repository,
                          None,
                          None,
                          CRYPTO_CONFIGURATION_UPDATE_MESSAGE,
                          &[PathBuf::from(CRYPTO_CONFIGURATION_PATH).as_path()])
            .unwrap();
    }
}
