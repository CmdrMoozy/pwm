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
use error::*;
#[cfg(feature = "piv")]
use piv;
use std::path::Path;

static IDENTIFIER_APPLICATION: &'static str = "pwm";
#[cfg(debug_assertions)]
static IDENTIFIER_NAME: &'static str = "pwm-debug";
#[cfg(not(debug_assertions))]
static IDENTIFIER_NAME: &'static str = "pwm";

lazy_static! {
    static ref IDENTIFIER: bdrck_config::Identifier = bdrck_config::Identifier {
        application: IDENTIFIER_APPLICATION.to_owned(),
        name: IDENTIFIER_NAME.to_owned(),
    };
}

pub static DEFAULT_REPOSITORY_KEY: &'static str = "default_repository";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Configuration {
    pub default_repository: Option<String>,
    #[cfg(feature = "piv")]
    pub piv: Option<piv::Configuration>,
}

pub struct SingletonHandle;

impl SingletonHandle {
    pub fn new(custom_path: Option<&Path>) -> Result<SingletonHandle> {
        bdrck_config::new(IDENTIFIER.clone(), Configuration::default(), custom_path)?;
        Ok(SingletonHandle {})
    }
}

impl Drop for SingletonHandle {
    fn drop(&mut self) {
        if let Err(e) = bdrck_config::remove::<Configuration>(&IDENTIFIER) {
            error!("Persisting configuration failed: {}", e);
        }
    }
}

pub fn set(key: &str, value: &str) -> Result<()> {
    let err = bdrck_config::instance_apply_mut(
        &IDENTIFIER,
        |instance: &mut bdrck_config::Configuration<Configuration>| -> Option<Error> {
            let mut config = instance.get().clone();
            if key == DEFAULT_REPOSITORY_KEY {
                config.default_repository = Some(value.to_owned());
            } else {
                return Some(Error::InvalidArgument(format_err!(
                    "Invalid configuration key '{}'",
                    key
                )));
            }
            instance.set(config);
            None
        },
    )?;

    match err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

pub fn get() -> Result<Configuration> {
    Ok(bdrck_config::get::<Configuration>(&IDENTIFIER)?)
}

pub fn get_value_as_str(key: &str) -> Result<String> {
    let config = get()?;
    if key == DEFAULT_REPOSITORY_KEY {
        Ok(match config.default_repository {
            Some(v) => v.clone(),
            None => String::new(),
        })
    } else {
        return Err(Error::InvalidArgument(format_err!(
            "Invalid configuration key '{}'",
            key
        )));
    }
}

pub fn reset() -> Result<()> {
    Ok(bdrck_config::reset::<Configuration>(&IDENTIFIER)?)
}
