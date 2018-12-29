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

use crate::configuration;
use crate::crypto::pwgen;
use crate::error::*;
use crate::repository::serde::{export_serialize, import_deserialize};
use crate::repository::Repository;
use crate::util::data::{end_user_display, load_file, SecretSlice};
use crate::util::{self, multiline_password_prompt, password_prompt};
use failure::format_err;
use std::fs::File;
use std::io;
use std::path::Path;

static NEW_PASSWORD_PROMPT: &'static str = "New password: ";
static MULTILINE_PASSWORD_PROMPT: &'static str = "Enter password data, until 'EOF' is read:";

pub(crate) fn config(key: Option<&str>, set: Option<&str>) -> Result<()> {
    if key.is_none() {
        if set.is_some() {
            return Err(Error::InvalidArgument(format_err!(
                "A 'key' must be provided when 'set'ting a configuration value."
            )));
        }

        println!(
            "{}",
            serde_json::to_string_pretty(&configuration::get().unwrap())?
        );
        return Ok(());
    }

    let key = key.unwrap();
    if let Some(set) = set {
        configuration::set(key, set).unwrap();
    }

    println!("{} = {}", key, configuration::get_value_as_str(key)?);

    Ok(())
}

pub(crate) fn init<P: AsRef<Path>>(path: P) -> Result<()> {
    let repository = Repository::new(path.as_ref(), true, None)?;
    println!(
        "Initialized repository: {}",
        repository.workdir().unwrap().display()
    );

    Ok(())
}

pub(crate) fn addkey<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut repository = Repository::new(path.as_ref(), false, None)?;
    repository.add_password_key(None)?;

    Ok(())
}

pub(crate) fn rmkey<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut repository = Repository::new(path.as_ref(), false, None)?;
    repository.remove_key(None)?;

    Ok(())
}

pub(crate) fn ls<RP: AsRef<Path>>(repository_path: RP, path: &str) -> Result<()> {
    let repository = Repository::new(repository_path, false, None)?;
    let path = repository.path(path)?;
    for entry in &repository.list(Some(&path))? {
        println!("{}", entry.to_str().unwrap());
    }

    Ok(())
}

fn print_stored_data(retrieved: &SecretSlice, force_binary: bool) -> Result<()> {
    let tty = bdrck::cli::isatty(bdrck::cli::Stream::Stdout);
    let display: Option<String> = end_user_display(retrieved, force_binary, tty);
    let bytes: &[u8] = display
        .as_ref()
        .map_or_else(|| &retrieved[..], |s| s.as_bytes());

    if tty {
        println!("{}", String::from_utf8_lossy(bytes));
    } else {
        use std::io::Write;
        let mut stdout = io::stdout();
        stdout.write_all(bytes)?;
    }

    Ok(())
}

pub(crate) fn get<RP: AsRef<Path>>(
    repository_path: RP,
    force_binary: bool,
    clipboard: bool,
    path: &str,
) -> Result<()> {
    let repository = Repository::new(repository_path.as_ref(), false, None)?;
    let path = repository.path(path)?;

    let retrieved = repository.read_decrypt(&path)?;

    match () {
        #[cfg(feature = "clipboard")]
        () => {
            if clipboard {
                util::clipboard::set_contents(&retrieved, force_binary)?;
            } else {
                print_stored_data(&retrieved, force_binary)?;
            }
        }

        #[cfg(not(feature = "clipboard"))]
        () => {
            print_stored_data(&retrieved, force_binary)?;
        }
    }

    Ok(())
}

pub(crate) fn set<RP: AsRef<Path>, KP: AsRef<Path>>(
    repository_path: RP,
    key_path: Option<KP>,
    multiline: bool,
    path: &str,
) -> Result<()> {
    let mut repository = Repository::new(repository_path.as_ref(), false, None)?;
    let path = repository.path(path)?;

    if key_path.is_some() && multiline {
        return Err(Error::InvalidArgument(format_err!(
            "The 'key_file' and 'multiline' options are mutually exclusive."
        )));
    }

    if let Some(key_path) = key_path {
        // The user wants to set the password using a key file.
        repository.write_encrypt(&path, load_file(key_path.as_ref())?)?;
    } else {
        // The user wants to set the password, but no key file was given, so prompt for
        // the password interactively.
        repository.write_encrypt(
            &path,
            match multiline {
                false => password_prompt(NEW_PASSWORD_PROMPT, true)?,
                true => multiline_password_prompt(MULTILINE_PASSWORD_PROMPT)?,
            },
        )?;
    }

    Ok(())
}

pub(crate) fn rm<RP: AsRef<Path>>(repository_path: RP, path: &str) -> Result<()> {
    let mut repository = Repository::new(repository_path.as_ref(), false, None)?;
    let path = repository.path(path)?;
    repository.remove(&path)?;
    Ok(())
}

pub(crate) fn generate(
    length: usize,
    exclude_letters: bool,
    exclude_numbers: bool,
    include_symbols: bool,
    custom_exclude: Option<&str>,
) -> Result<()> {
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

    println!(
        "{}",
        end_user_display(
            pwgen::generate_password(length, charsets.as_slice(), custom_exclude.as_slice())?
                .as_slice(),
            false,
            false
        )
        .unwrap()
    );

    Ok(())
}

pub(crate) fn export<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut repository = Repository::new(path.as_ref(), false, None)?;
    println!("{}", export_serialize(&mut repository)?);
    Ok(())
}

pub(crate) fn import<RP: AsRef<Path>, IP: AsRef<Path>>(
    repository_path: RP,
    input_path: IP,
) -> Result<()> {
    use std::io::Read;

    let mut repository = Repository::new(repository_path.as_ref(), false, None)?;

    let mut input = String::new();
    let mut f = File::open(input_path.as_ref())?;
    f.read_to_string(&mut input)?;

    import_deserialize(&mut repository, input.as_str())?;

    Ok(())
}
