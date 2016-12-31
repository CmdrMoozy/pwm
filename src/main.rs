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

use std::boxed::Box;
use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

extern crate bdrck_params;
use ::bdrck_params::argument::Argument;
use ::bdrck_params::command::Command;
use ::bdrck_params::command::ExecutableCommand;
use ::bdrck_params::main_impl::main_impl_multiple_commands;
use ::bdrck_params::option::Option;

extern crate pwm;

fn config(_: &HashMap<&str, String>, _: &HashMap<&str, bool>, _: &HashMap<&str, Vec<String>>) {}

fn init(_: &HashMap<&str, String>, _: &HashMap<&str, bool>, _: &HashMap<&str, Vec<String>>) {}

fn ls(_: &HashMap<&str, String>, _: &HashMap<&str, bool>, _: &HashMap<&str, Vec<String>>) {}

fn pw(_: &HashMap<&str, String>, _: &HashMap<&str, bool>, _: &HashMap<&str, Vec<String>>) {}

fn main() {
    pwm::init().ok().unwrap();

    let commands = vec![
        Command::new("config".to_owned(), "Get or set a configuration value".to_owned(),
            vec![
                Option::optional("set", "Set the key to this new value", Some('s')),
                Option::optional("key", "The specific key to view / set", Some('k')),
            ],
            vec![],
            false).unwrap(),
        Command::new("init".to_owned(), "Initialize a new pwm repository".to_owned(),
            vec![
                Option::optional("repository",
                    "The path to the repository to initialize", Some('r')),
            ],
            vec![],
            false).unwrap(),
        Command::new("ls".to_owned(), "List passwords stored in a pwm repository".to_owned(),
            vec![
                Option::optional("repository",
                    "The path to the repository to initialize", Some('r')),
            ],
            vec![
                Argument {
                    name: "path".to_owned(),
                    help: "The path to list, relative to the repository's root".to_owned(),
                    default_value: Some(vec!["/".to_owned()]),
                },
            ],
            false).unwrap(),
        Command::new("pw".to_owned(), "Get or set a password from a pwm repository".to_owned(),
            vec![
                Option::optional("repository",
                    "The path to the repository to initialize", Some('r')),
                Option::flag("set", "Set this password using a command-line prompt", Some('s')),
                Option::optional("key", "Set this password using a key file", Some('k')),
            ],
            vec![
                Argument {
                    name: "path".to_owned(),
                    help: "The path to get / set, relative to the repository's root".to_owned(),
                    default_value: None,
                },
            ],
            false).unwrap(),
    ];

    let callbacks: Vec<Box<FnMut(&HashMap<&str, String>,
                                 &HashMap<&str, bool>,
                                 &HashMap<&str, Vec<String>>)>> = vec![
        Box::new(config),
        Box::new(init),
        Box::new(ls),
        Box::new(pw),
    ];

    main_impl_multiple_commands(commands.iter()
        .zip(callbacks.into_iter())
        .map(|cp| ExecutableCommand::new(&cp.0, cp.1))
        .collect());
}
