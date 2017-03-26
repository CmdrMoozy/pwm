use bdrck_config::configuration as bdrck_config;
use error::{Error, Result};
use std::path::Path;

pub static DEFAULT_REPOSITORY_KEY: &'static str = "default_repository";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub default_repository: Option<String>,
}

lazy_static! {
    static ref DEFAULT_CONFIGURATION: Configuration = Configuration {
        default_repository: None,
    };
}

fn get_identifier() -> &'static bdrck_config::Identifier {
    lazy_static! {
        static ref IDENTIFIER: bdrck_config::Identifier = bdrck_config::Identifier {
            application: "pwm".to_owned(),
            name: "pwm".to_owned(),
        };

        static ref DEBUG_IDENTIFIER: bdrck_config::Identifier = bdrck_config::Identifier {
            application: "pwm".to_owned(),
            name: "pwm-debug".to_owned(),
        };
    }

    if cfg!(debug_assertions) {
        &DEBUG_IDENTIFIER
    } else {
        &IDENTIFIER
    }
}

pub struct SingletonHandle;

impl SingletonHandle {
    pub fn new(custom_path: Option<&Path>) -> Result<SingletonHandle> {
        try!(bdrck_config::new(get_identifier().clone(),
                               DEFAULT_CONFIGURATION.clone(),
                               custom_path));
        Ok(SingletonHandle {})
    }
}

impl Drop for SingletonHandle {
    fn drop(&mut self) {
        if let Err(e) = bdrck_config::remove::<Configuration>(get_identifier()) {
            error!("Persisting configuration failed: {}", e);
        }
    }
}

pub fn set(key: &str, value: &str) -> Result<()> {
    let err = try!(bdrck_config::instance_apply_mut(get_identifier(),
        |instance: &mut bdrck_config::Configuration<Configuration>| -> Option<Error> {
        let mut config = instance.get().clone();
        if key == DEFAULT_REPOSITORY_KEY {
            config.default_repository = Some(value.to_owned());
        } else {
            let e: Error = format!("Invalid configuration key '{}'", key).into();
            return Some(e);
        }
        instance.set(config);
        None
    }));

    match err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

pub fn get() -> Result<Configuration> {
    Ok(try!(bdrck_config::get::<Configuration>(get_identifier())))
}

pub fn get_value_as_str(key: &str) -> Result<String> {
    let config = try!(get());
    if key == DEFAULT_REPOSITORY_KEY {
        Ok(match config.default_repository {
            Some(v) => v.clone(),
            None => String::new(),
        })
    } else {
        bail!("Invalid configuration key '{}'", key);
    }
}

pub fn reset() -> Result<()> { Ok(try!(bdrck_config::reset::<Configuration>(get_identifier()))) }
