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

use bdrck::configuration as bdrck_config;
use error::Result;
use sodiumoxide::crypto::pwhash;
use sodiumoxide::crypto::pwhash::{MemLimit, OpsLimit, Salt, SALTBYTES};
use std::path::Path;

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

    pub fn get_salt(&self) -> Salt { Salt(self.salt) }

    pub fn get_mem_limit(&self) -> MemLimit { MemLimit(self.mem_limit) }

    pub fn get_ops_limit(&self) -> OpsLimit { OpsLimit(self.ops_limit) }
}

impl Default for Configuration {
    fn default() -> Configuration {
        Self::new(
            pwhash::gen_salt(),
            pwhash::MEMLIMIT_INTERACTIVE,
            pwhash::OPSLIMIT_INTERACTIVE,
        )
    }
}

impl PartialEq for Configuration {
    fn eq(&self, other: &Configuration) -> bool {
        self.salt == other.salt && self.mem_limit == other.mem_limit
            && self.ops_limit == other.ops_limit
    }
}

impl Eq for Configuration {}

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
        bdrck_config::new(
            instance.identifier.clone(),
            Configuration::default(),
            Some(path.as_ref()),
        )?;
        Ok(instance)
    }

    pub fn close(self) -> Result<()> {
        bdrck_config::remove::<Configuration>(&self.identifier)?;
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
