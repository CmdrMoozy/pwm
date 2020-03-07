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

use crate::error::*;
use crate::util::data::Secret;
use crate::util::unwrap_password_or_prompt;
use bdrck::configuration as bdrck_config;
use bdrck::crypto::key::*;
use serde_derive::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Configuration {
    salt: Salt,
    mem_limit: usize,
    ops_limit: usize,
}

impl Configuration {
    pub fn new(salt: Salt, mem_limit: usize, ops_limit: usize) -> Configuration {
        Configuration {
            salt: salt,
            mem_limit: mem_limit,
            ops_limit: ops_limit,
        }
    }

    pub fn get_salt(&self) -> Salt {
        self.salt.clone()
    }

    pub fn get_mem_limit(&self) -> usize {
        self.mem_limit
    }

    pub fn get_ops_limit(&self) -> usize {
        self.ops_limit
    }

    pub fn get_password_key(
        &self,
        password: Option<Secret>,
        prompt: &str,
        confirm: bool,
    ) -> Result<Box<dyn AbstractKey>> {
        let password = unwrap_password_or_prompt(password, prompt, confirm)?;
        Ok(Box::new(Key::new_password(
            password.as_slice(),
            &self.salt,
            self.ops_limit,
            self.mem_limit,
        )?))
    }
}

impl Default for Configuration {
    fn default() -> Configuration {
        Self::new(
            Salt::default(),
            MEM_LIMIT_INTERACTIVE,
            OPS_LIMIT_INTERACTIVE,
        )
    }
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

    pub fn reset(&self) {
        bdrck_config::reset::<Configuration>(&self.identifier).unwrap()
    }
}
