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
pub struct Configuration {
    salt: [u8; SALTBYTES],
    mem_limit: usize,
    ops_limit: usize,
}

impl Configuration {
    pub fn new(salt: Salt, mem_limit: MemLimit, ops_limit: OpsLimit) -> Configuration {
        Configuration {
            salt: salt.0,
            mem_limit: mem_limit.0,
            ops_limit: ops_limit.0,
        }
    }

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

impl PartialEq for Configuration {
    fn eq(&self, other: &Configuration) -> bool {
        self.salt == other.salt && self.mem_limit == other.mem_limit &&
        self.ops_limit == other.ops_limit
    }
}

impl Eq for Configuration {}

lazy_static! {
    static ref DEFAULT_CONFIGURATION: Configuration = Configuration {
        salt: pwhash::gen_salt().0,
        mem_limit: pwhash::MEMLIMIT_INTERACTIVE.0,
        ops_limit: pwhash::OPSLIMIT_INTERACTIVE.0,
    };
}

pub struct ConfigurationInstance {
    identifier: bdrck_config::Identifier,
}

impl ConfigurationInstance {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ConfigurationInstance> {
        let instance = ConfigurationInstance {
            identifier: bdrck_config::Identifier {
                application: "pwm".to_owned(),
                name: path.as_ref().to_string_lossy().into_owned(),
            },
        };
        try!(bdrck_config::new(instance.identifier.clone(),
                               DEFAULT_CONFIGURATION.clone(),
                               Some(path.as_ref())));
        Ok(instance)
    }

    pub fn close(self) -> Result<()> {
        try!(bdrck_config::remove::<Configuration>(&self.identifier));
        Ok(())
    }

    pub fn get(&self) -> Configuration {
        bdrck_config::get::<Configuration>(&self.identifier).unwrap()
    }

    pub fn set(&self, config: Configuration) {
        bdrck_config::set(&self.identifier, config).unwrap()
    }

    pub fn reset(&self) { bdrck_config::reset::<Configuration>(&self.identifier).unwrap() }
}
