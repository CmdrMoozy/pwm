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

use crate::crypto::key::PwmKey;
use crate::error::*;
use crate::secret::Secret;
use crate::util::unwrap_password_or_prompt;
use bdrck::configuration as bdrck_config;
use bdrck::crypto::key::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[cfg(not(feature = "piv"))]
fn deserialize_piv_keys_panic<'de, D: serde::Deserializer<'de>, T>(
    _: D,
) -> std::result::Result<T, D::Error> {
    panic!("PIV feature is disabled; refusing to load PIV configuration");
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    salt: Salt,
    mem_limit: usize,
    ops_limit: usize,

    #[cfg(feature = "piv")]
    // Default to an empty Vec if the structure didn't previously have this.
    #[serde(default)]
    piv_keys: Vec<crate::piv::util::PivKeyAssociation>,

    #[cfg(not(feature = "piv"))]
    // We must default in order to load structures which omit this (all should).
    #[serde(default)]
    // Don't write this field out when serializing this structure (it's just a
    // placeholder).
    #[serde(skip_serializing)]
    // If we actually find a structure with this field, instead of dserializing
    // it, just panic instead (it's not supported without the PIV feature).
    #[serde(deserialize_with = "deserialize_piv_keys_panic")]
    piv_keys: std::marker::PhantomData<()>,
}

impl Configuration {
    pub fn new(salt: Salt, mem_limit: usize, ops_limit: usize) -> Configuration {
        Configuration {
            salt: salt,
            mem_limit: mem_limit,
            ops_limit: ops_limit,

            #[cfg(feature = "piv")]
            piv_keys: Vec::new(),
            #[cfg(not(feature = "piv"))]
            piv_keys: std::marker::PhantomData,
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

    #[cfg(feature = "piv")]
    pub(crate) fn get_piv_keys(&self) -> &[crate::piv::util::PivKeyAssociation] {
        self.piv_keys.as_slice()
    }

    #[cfg(feature = "piv")]
    pub(crate) fn set_piv_keys(&mut self, keys: Vec<crate::piv::util::PivKeyAssociation>) {
        self.piv_keys = keys;
    }

    #[cfg(feature = "piv")]
    pub(crate) fn add_piv_key(&mut self, assoc: crate::piv::util::PivKeyAssociation) {
        self.piv_keys.push(assoc);
    }

    pub fn get_password_key(
        &self,
        password: Option<Secret>,
        prompt: &str,
        confirm: bool,
    ) -> Result<impl AbstractKey<Error = Error>> {
        let password = unwrap_password_or_prompt(password, prompt, confirm)?;
        let key = Key::new_password(
            password.as_slice(),
            &self.salt,
            self.ops_limit,
            self.mem_limit,
        )?;
        Ok(PwmKey::from(key))
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
