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

use bdrck_config::configuration as bdrck_config;
use ::crypto::key::Key;
use ::error::Result;
use sodiumoxide::crypto::pwhash;
use sodiumoxide::crypto::pwhash::{MemLimit, OpsLimit, Salt, SALTBYTES};
use std::path::Path;
use ::util::data::SensitiveData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CryptoConfiguration {
    salt: [u8; SALTBYTES],
    mem_limit: usize,
    ops_limit: usize,
}

impl CryptoConfiguration {
    pub fn get_salt(&self) -> Salt { Salt(self.salt.clone()) }

    pub fn get_mem_limit(&self) -> MemLimit { MemLimit(self.mem_limit) }

    pub fn get_ops_limit(&self) -> OpsLimit { OpsLimit(self.ops_limit) }

    pub fn build_key(&self, password: SensitiveData) -> Result<Key> {
        Key::new(password,
                 Some(self.get_salt()),
                 Some(self.get_ops_limit()),
                 Some(self.get_mem_limit()))
    }
}

lazy_static! {
    static ref DEFAULT_CRYPTO_CONFIGURATION: CryptoConfiguration = CryptoConfiguration {
        salt: pwhash::gen_salt().0,
        mem_limit: pwhash::MEMLIMIT_INTERACTIVE.0,
        ops_limit: pwhash::OPSLIMIT_INTERACTIVE.0,
    };
}

pub struct CryptoConfigurationInstance {
    identifier: bdrck_config::Identifier,
}

impl CryptoConfigurationInstance {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<CryptoConfigurationInstance> {
        let instance = CryptoConfigurationInstance {
            identifier: bdrck_config::Identifier {
                application: "pwm".to_owned(),
                name: path.as_ref().to_string_lossy().into_owned(),
            },
        };
        try!(bdrck_config::new(instance.identifier.clone(),
                               DEFAULT_CRYPTO_CONFIGURATION.clone(),
                               Some(path.as_ref())));
        Ok(instance)
    }

    pub fn close(self) -> Result<()> {
        try!(bdrck_config::remove::<CryptoConfiguration>(&self.identifier));
        Ok(())
    }

    pub fn get(&self) -> Result<CryptoConfiguration> {
        Ok(try!(bdrck_config::get::<CryptoConfiguration>(&self.identifier)))
    }

    pub fn set(&self, config: CryptoConfiguration) -> Result<()> {
        Ok(try!(bdrck_config::set(&self.identifier, config)))
    }

    pub fn reset(&self) -> Result<()> {
        Ok(try!(bdrck_config::reset::<CryptoConfiguration>(&self.identifier)))
    }
}
