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

use crate::cli::util::get_repository_path;
use crate::configuration;
use crate::crypto::pwgen;
use crate::error::*;
use crate::output::{output_secret, InputEncoding, OutputMethod};
use crate::repository::serde::{export_serialize, import_deserialize};
use crate::repository::Repository;
use crate::util::{self, multiline_password_prompt, password_prompt};
use flaggy::*;
use std::fs::File;
use std::path::PathBuf;

static NEW_PASSWORD_PROMPT: &'static str = "New password: ";
static MULTILINE_PASSWORD_PROMPT: &'static str = "Enter password data, until 'EOF' is read:";

#[command_callback]
pub(crate) fn config(key: Option<String>, set: Option<String>) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    if key.is_none() {
        if set.is_some() {
            bail!("a 'key' must be provided when 'set'ting a configuration value");
        }

        println!(
            "{}",
            serde_json::to_string_pretty(&configuration::get().unwrap())?
        );
        return Ok(());
    }

    let key = key.unwrap();
    if let Some(set) = set {
        configuration::set(&key, &set).unwrap();
    }

    println!("{} = {}", key, configuration::get_value_as_str(&key)?);

    Ok(())
}

#[command_callback]
pub(crate) fn init(repository: Option<PathBuf>) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let repository = Repository::new(&repository, true, None)?;
    println!(
        "Initialized repository: {}",
        repository.workdir().unwrap().display()
    );

    Ok(())
}

#[command_callback]
pub(crate) fn addkey(repository: Option<PathBuf>) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let mut repository = Repository::new(&repository, false, None)?;
    repository.add_password_key(None)?;

    Ok(())
}

#[command_callback]
pub(crate) fn rmkey(repository: Option<PathBuf>) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let mut repository = Repository::new(&repository, false, None)?;
    repository.remove_password_key(None)?;

    Ok(())
}

#[command_callback]
pub(crate) fn ls(repository: Option<PathBuf>, path_prefix: String) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let repository = Repository::new(&repository, false, None)?;
    let path = repository.path(path_prefix)?;
    for entry in &repository.list(Some(&path))? {
        println!("{}", entry.to_str().unwrap());
    }

    Ok(())
}

#[command_callback]
pub(crate) fn get(
    repository: Option<PathBuf>,
    binary: bool,
    output_method: OutputMethod,
    path: String,
) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let repository = Repository::new(&repository, false, None)?;
    let path = repository.path(path)?;
    output_secret(
        &repository.read_decrypt(&path)?,
        match binary {
            false => InputEncoding::Auto,
            true => InputEncoding::Binary,
        },
        output_method,
    )?;
    Ok(())
}

#[command_callback]
pub(crate) fn set(
    repository: Option<PathBuf>,
    key_file: Option<PathBuf>,
    multiline: bool,
    path: String,
) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let mut repository = Repository::new(&repository, false, None)?;
    let path = repository.path(path)?;

    if key_file.is_some() && multiline {
        bail!("the 'key_file' and 'multiline' options are mutually exclusive");
    }

    if let Some(key_file) = key_file {
        // The user wants to set the password using a key file.
        repository.write_encrypt(&path, util::secret::load_file(&key_file)?, None)?;
    } else {
        // The user wants to set the password, but no key file was given, so prompt for
        // the password interactively.
        repository.write_encrypt(
            &path,
            match multiline {
                false => password_prompt(NEW_PASSWORD_PROMPT, true)?,
                true => multiline_password_prompt(MULTILINE_PASSWORD_PROMPT)?,
            },
            None,
        )?;
    }

    Ok(())
}

#[command_callback]
pub(crate) fn rm(repository: Option<PathBuf>, path: String) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let mut repository = Repository::new(&repository, false, None)?;
    let path = repository.path(path)?;
    repository.remove(&path)?;
    Ok(())
}

#[command_callback]
pub(crate) fn generate(
    password_length: usize,
    exclude_letters: bool,
    exclude_numbers: bool,
    include_symbols: bool,
    custom_exclude: Option<String>,
) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let mut charsets: Vec<pwgen::CharacterSet> = Vec::new();
    if !exclude_letters {
        charsets.push(pwgen::CharacterSet::Letters);
    }
    if !exclude_numbers {
        charsets.push(pwgen::CharacterSet::Numbers);
    }
    if include_symbols {
        charsets.push(pwgen::CharacterSet::Symbols);
    }
    let custom_exclude: Vec<char> = custom_exclude.map_or(vec![], |x| x.chars().collect());

    output_secret(
        &pwgen::generate_password(
            password_length,
            charsets.as_slice(),
            custom_exclude.as_slice(),
        )?,
        InputEncoding::Auto,
        OutputMethod::Stdout,
    )?;

    Ok(())
}

#[command_callback]
pub(crate) fn export(repository: Option<PathBuf>) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let mut repository = Repository::new(&repository, false, None)?;
    println!("{}", export_serialize(&mut repository)?);
    Ok(())
}

#[command_callback]
pub(crate) fn import(repository: Option<PathBuf>, input: PathBuf) -> Result<()> {
    use std::io::Read;

    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let mut repository = Repository::new(&repository, false, None)?;

    let mut input_data = String::new();
    let mut f = File::open(&input)?;
    f.read_to_string(&mut input_data)?;

    import_deserialize(&mut repository, input_data.as_str())?;

    Ok(())
}
