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

use std::collections::HashMap;
use std::fs::File;
use std::option::Option as Optional;

extern crate bdrck_params;
use ::bdrck_params::argument::Argument;
use ::bdrck_params::command::Command;
use ::bdrck_params::command::ExecutableCommand;
use ::bdrck_params::main_impl::main_impl_multiple_commands;
use ::bdrck_params::option::Option;

#[macro_use]
extern crate log;

extern crate pwm_lib;
use pwm_lib::configuration;
use pwm_lib::error::{Error, ErrorKind, Result};
use pwm_lib::repository::{Path, Repository};
use pwm_lib::util::data::SensitiveData;
use pwm_lib::util::password_prompt;

extern crate serde_json;

static NEW_PASSWORD_PROMPT: &'static str = "New password: ";

fn init_pwm() -> Result<configuration::SingletonHandle> {
    try!(pwm_lib::init());
    Ok(try!(configuration::SingletonHandle::new()))
}

fn get_repository_path(options: &HashMap<String, String>) -> Result<String> {
    match options.get("repository").or(try!(configuration::get()).default_repository.as_ref()) {
        Some(p) => Ok(p.clone()),
        None => {
            Err(Error::new(ErrorKind::Parameters {
                description: "No repository path specified. Try the 'repository' command option, \
                              or setting the 'default_repository' configuration key."
                    .to_owned(),
            }))
        },
    }
}

fn config(options: HashMap<String, String>,
          _: HashMap<String, bool>,
          _: HashMap<String, Vec<String>>)
          -> Result<()> {
    let _handle = try!(init_pwm());

    let k: Optional<&String> = options.get("key");
    let s: Optional<&String> = options.get("set");

    if k.is_none() {
        if s.is_some() {
            return Err(Error::new(ErrorKind::Parameters {
                description: "A 'key' must be provided when 'set'ting a configuration value."
                    .to_owned(),
            }));
        }

        info!("{}",
              serde_json::to_string_pretty(&configuration::get().unwrap()).unwrap());
        return Ok(());
    }

    let key = k.unwrap();
    if let Some(set) = s {
        configuration::set(key.as_str(), set.as_str()).unwrap();
    }

    info!("{} = {}",
          key,
          try!(configuration::get_value_as_str(key.as_str())));

    Ok(())
}

fn init(options: HashMap<String, String>,
        _: HashMap<String, bool>,
        _: HashMap<String, Vec<String>>)
        -> Result<()> {
    let _handle = try!(init_pwm());

    let repository = try!(Repository::new(try!(get_repository_path(&options)), true, None));
    info!("Initialized repository: {}",
          repository.workdir().unwrap().display());

    Ok(())
}

fn ls(options: HashMap<String, String>,
      _: HashMap<String, bool>,
      arguments: HashMap<String, Vec<String>>)
      -> Result<()> {
    let _handle = try!(init_pwm());

    let repository = try!(Repository::new(try!(get_repository_path(&options)), false, None));
    let path = try!(Path::from_repository(&repository, &arguments.get("path").unwrap()[0]));
    for entry in try!(repository.list(&path)).iter() {
        info!("{}", entry.to_str().unwrap());
    }

    Ok(())
}

fn pw(options: HashMap<String, String>,
      flags: HashMap<String, bool>,
      arguments: HashMap<String, Vec<String>>)
      -> Result<()> {
    let _handle = try!(init_pwm());

    let repository = try!(Repository::new(try!(get_repository_path(&options)), false, None));
    let path = try!(Path::from_repository(&repository, &arguments.get("path").unwrap()[0]));

    let set: bool = flags.get("set").cloned().unwrap_or(false);
    let k = options.get("key");

    if set && k.is_none() {
        // The user wants to set the password, but no key file was given, so prompt for
        // the password interactively.
        try!(repository.write_encrypt(&path, try!(password_prompt(NEW_PASSWORD_PROMPT, true))));
    } else if let Some(key) = k {
        // The user wants to set the password using a key file.
        let mut key_file = try!(File::open(key));
        try!(repository.write_encrypt(&path, try!(SensitiveData::from_file(&mut key_file))));
    } else {
        // The user wants to retrieve the password, instead of set it.
        info!("{}", try!(repository.read_decrypt(&path)));
    }

    Ok(())
}

fn main() {
    main_impl_multiple_commands(vec![ExecutableCommand::new(Command::new("config",
                                                                         "Get or set a \
                                                                          configuration value",
                                                                         vec![
                Option::optional("set", "Set the key to this new value", Some('s')),
                Option::optional("key", "The specific key to view / set", Some('k')),
            ],
                                                                         vec![],
                                                                         false)
                                                                .unwrap(),
                                                            Box::new(config)),
                                     ExecutableCommand::new(Command::new("init",
                                                                         "Initialize a new \
                                                                          pwm repository",
                                                                         vec![
                Option::optional("repository",
                    "The path to the repository to initialize", Some('r')),
            ],
                                                                         vec![],
                                                                         false)
                                                                .unwrap(),
                                                            Box::new(init)),
                                     ExecutableCommand::new(Command::new("ls",
                                                                         "List passwords \
                                                                          stored in a pwm \
                                                                          repository",
                                                                         vec![
                Option::optional("repository",
                    "The path to the repository to initialize", Some('r')),
            ],
                                                                         vec![
                Argument::new("path",
                              "The path to list, relative to the repository's root",
                              Some(vec!["".to_owned()])),
            ],
                                                                         false)
                                                                .unwrap(),
                                                            Box::new(ls)),
                                     ExecutableCommand::new(Command::new("pw",
                                                                         "Get or set a \
                                                                          password from a pwm \
                                                                          repository",
                                                                         vec![
                Option::optional("repository",
                    "The path to the repository to initialize", Some('r')),
                Option::flag("set", "Set this password using a command-line prompt", Some('s')),
                Option::optional("key", "Set this password using a key file", Some('k')),
            ],
                                                                         vec![
                Argument::new("path",
                              "The path to get / set, relative to the repository's root",
                              None),
            ],
                                                                         false)
                                                                .unwrap(),
                                                            Box::new(pw))]);
}
