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
use ::error::{Error, ErrorKind, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub default_repository: Option<String>,
}

lazy_static! {
    static ref IDENTIFIER: bdrck_config::Identifier = bdrck_config::Identifier {
        application: "pwm".to_owned(),
        name: "pwm".to_owned(),
    };

    static ref DEFAULT_CONFIGURATION: Configuration = Configuration {
        default_repository: None,
    };
}

pub struct SingletonHandle;

impl SingletonHandle {
    pub fn new() -> Result<SingletonHandle> {
        try!(bdrck_config::new(IDENTIFIER.clone(), DEFAULT_CONFIGURATION.clone(), None));
        Ok(SingletonHandle {})
    }
}

impl Drop for SingletonHandle {
    fn drop(&mut self) { let _ = bdrck_config::remove::<Configuration>(&IDENTIFIER); }
}

pub fn set(key: &str, value: &str) -> Result<()> {
    let err = try!(bdrck_config::instance_apply_mut(&IDENTIFIER,
        |instance: &mut bdrck_config::Configuration<Configuration>| -> Option<Error> {
        let mut config = instance.get().clone();
        match key {
            "default_repository" => config.default_repository = Some(value.to_owned()),
            _ => return Some(Error::new(ErrorKind::Configuration {
                cause: format!("Invalid configuration key '{}'", key)
            })),
        };
        instance.set(config);
        None
    }));

    match err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

pub fn get() -> Result<Configuration> { Ok(try!(bdrck_config::get(&IDENTIFIER))) }

pub fn reset() -> Result<()> { Ok(try!(bdrck_config::reset::<Configuration>(&IDENTIFIER))) }
