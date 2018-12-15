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

mod commands;
mod impls;
pub mod util;

use crate::error::*;
use bdrck::flags::command::{Command, CommandCallback};
use bdrck::flags::spec::{Spec, Specs};
use bdrck::flags::value::Values;
use lazy_static::lazy_static;

pub(crate) fn to_command_fn<F: 'static + FnMut(Values) -> Result<()>>(
    mut f: F,
) -> CommandCallback<'static, Error> {
    Box::new(move |values| {
        let _handle = crate::init_with_configuration()?;
        f(values)
    })
}

lazy_static! {
    pub static ref REPOSITORY_SPEC: Spec = Spec::optional(
        "repository",
        "The path to the pwm repository to use",
        Some('r')
    );
    pub static ref PATH_SPEC: Spec = Spec::positional(
        "path",
        "The saved password path, relative to the repository's root",
        None,
        false
    )
    .unwrap();
    pub static ref PATH_PREFIX_SPEC: Spec = Spec::positional(
        "path_prefix",
        "The saved password path prefix, relative to the repository's root",
        Some(&[""]),
        false
    )
    .unwrap();
    pub static ref CONFIG_KEY_SPEC: Spec =
        Spec::optional("key", "The specific key to get or set", Some('k'));
    pub static ref CONFIG_SET_SPEC: Spec =
        Spec::optional("set", "The new value to set the key to", Some('s'));
    pub static ref GET_BINARY_SPEC: Spec = Spec::boolean(
        "binary",
        "Treat the saved password or key as binary data",
        Some('b')
    );
    pub static ref GET_CLIPBOARD_SPEC: Spec = Spec::boolean(
        "clipboard",
        "Copy the saved password or key to the clipboard",
        Some('c')
    );
    pub static ref SET_KEY_FILE_SPEC: Spec = Spec::optional(
        "key_file",
        "Store a key file instead of a password",
        Some('k')
    );
    pub static ref SET_MULTILINE_SPEC: Spec = Spec::boolean(
        "multiline",
        "Read multiple lines of input data, until 'EOF'",
        Some('m')
    );
    pub static ref GENERATE_PASSWORD_LENGTH_SPEC: Spec = Spec::required(
        "password_length",
        "The length of the password to generate",
        Some('l'),
        Some(
            crate::crypto::pwgen::RECOMMENDED_MINIMUM_PASSWORD_LENGTH
                .to_string()
                .as_str()
        )
    );
    pub static ref GENERATE_EXCLUDE_LETTERS_SPEC: Spec = Spec::boolean(
        "exclude_letters",
        "Exclude letters from the password",
        Some('A')
    );
    pub static ref GENERATE_EXCLUDE_NUMBERS_SPEC: Spec = Spec::boolean(
        "exclude_numbers",
        "Exclude numbers from the password",
        Some('N')
    );
    pub static ref GENERATE_INCLUDE_SYMBOLS_SPEC: Spec = Spec::boolean(
        "include_symbols",
        "Include symbols in the password",
        Some('s')
    );
    pub static ref GENERATE_CUSTOM_EXCLUDE_SPEC: Spec = Spec::optional(
        "custom_exclude",
        "Exclute a custom set of characters",
        Some('x')
    );
    pub static ref IMPORT_INPUT_SPEC: Spec =
        Spec::required("input", "The input file to import from", Some('i'), None);
}

pub fn build_config_command() -> Command<'static, Error> {
    Command::new(
        "config",
        "Get or set a configuration value",
        Specs::new(vec![CONFIG_KEY_SPEC.clone(), CONFIG_SET_SPEC.clone()]).unwrap(),
        to_command_fn(commands::config),
    )
}

pub fn build_init_command() -> Command<'static, Error> {
    Command::new(
        "init",
        "Initialize a new pwm repository",
        Specs::new(vec![REPOSITORY_SPEC.clone()]).unwrap(),
        to_command_fn(commands::init),
    )
}

pub fn build_addkey_command() -> Command<'static, Error> {
    Command::new(
        "addkey",
        "Add a new master key to an existing repository",
        Specs::new(vec![REPOSITORY_SPEC.clone()]).unwrap(),
        to_command_fn(commands::addkey),
    )
}

pub fn build_rmkey_command() -> Command<'static, Error> {
    Command::new(
        "rmkey",
        "Remove an existing master key from an existing repository",
        Specs::new(vec![REPOSITORY_SPEC.clone()]).unwrap(),
        to_command_fn(commands::rmkey),
    )
}

pub fn build_ls_command() -> Command<'static, Error> {
    Command::new(
        "ls",
        "List passwords stored in a pwm repository",
        Specs::new(vec![REPOSITORY_SPEC.clone(), PATH_PREFIX_SPEC.clone()]).unwrap(),
        to_command_fn(commands::ls),
    )
}

pub fn build_get_command() -> Command<'static, Error> {
    Command::new(
        "get",
        "Retrieve a password or key from a pwm repository",
        Specs::new(vec![
            REPOSITORY_SPEC.clone(),
            GET_BINARY_SPEC.clone(),
            #[cfg(feature = "clipboard")]
            GET_CLIPBOARD_SPEC.clone(),
            PATH_SPEC.clone(),
        ])
        .unwrap(),
        to_command_fn(commands::get),
    )
}

pub fn build_set_command() -> Command<'static, Error> {
    Command::new(
        "set",
        "Store a password or key in a pwm repository",
        Specs::new(vec![
            REPOSITORY_SPEC.clone(),
            SET_KEY_FILE_SPEC.clone(),
            SET_MULTILINE_SPEC.clone(),
            PATH_SPEC.clone(),
        ])
        .unwrap(),
        to_command_fn(commands::set),
    )
}

pub fn build_rm_command() -> Command<'static, Error> {
    Command::new(
        "rm",
        "Remove a password or key from a pwm repository",
        Specs::new(vec![REPOSITORY_SPEC.clone(), PATH_SPEC.clone()]).unwrap(),
        to_command_fn(commands::rm),
    )
}

pub fn build_generate_command() -> Command<'static, Error> {
    Command::new(
        "generate",
        "Generate a random password",
        Specs::new(vec![
            GENERATE_PASSWORD_LENGTH_SPEC.clone(),
            GENERATE_EXCLUDE_LETTERS_SPEC.clone(),
            GENERATE_EXCLUDE_NUMBERS_SPEC.clone(),
            GENERATE_INCLUDE_SYMBOLS_SPEC.clone(),
            GENERATE_CUSTOM_EXCLUDE_SPEC.clone(),
        ])
        .unwrap(),
        to_command_fn(commands::generate),
    )
}

pub fn build_export_command() -> Command<'static, Error> {
    Command::new(
        "export",
        "Export all stored passwords as plaintext JSON for backup purposes",
        Specs::new(vec![REPOSITORY_SPEC.clone()]).unwrap(),
        to_command_fn(commands::export),
    )
}

pub fn build_import_command() -> Command<'static, Error> {
    Command::new(
        "import",
        "Import stored passwords previously 'export'ed",
        Specs::new(vec![REPOSITORY_SPEC.clone(), IMPORT_INPUT_SPEC.clone()]).unwrap(),
        to_command_fn(commands::import),
    )
}
