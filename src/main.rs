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

use std::fs::File;
use std::io;
use std::option::Option as Optional;

extern crate bdrck;
use bdrck::flags::*;

#[macro_use]
extern crate error_chain;

extern crate isatty;

#[macro_use]
extern crate log;

extern crate pwm_lib;
use pwm_lib::configuration;
use pwm_lib::crypto::pwgen;
use pwm_lib::error::Result;
use pwm_lib::repository::Repository;
use pwm_lib::repository::serde::{export_serialize, import_deserialize};
use pwm_lib::util::{multiline_password_prompt, password_prompt};
use pwm_lib::util::data::SensitiveData;

extern crate serde_json;

static NEW_PASSWORD_PROMPT: &'static str = "New password: ";
static MULTILINE_PASSWORD_PROMPT: &'static str = "Enter password data, until 'EOF' is read:";

fn init_pwm() -> Result<configuration::SingletonHandle> {
    pwm_lib::init()?;
    Ok(configuration::SingletonHandle::new(None)?)
}

fn get_repository_path(values: &Values) -> Result<String> {
    let config = configuration::get()?;
    match values
        .get_single("repository")
        .or_else(|| config.default_repository.as_ref().map(|dr| dr.as_str()))
    {
        Some(p) => Ok(p.to_owned()),
        None => bail!(
            "No repository path specified. Try the 'repository' command option, or setting \
             the 'default_repository' configuration key."
        ),
    }
}

fn config(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let k = values.get_single("key");
    let s = values.get_single("set");
    if k.is_none() {
        if s.is_some() {
            bail!("A 'key' must be provided when 'set'ting a configuration value.");
        }

        println!(
            "{}",
            serde_json::to_string_pretty(&configuration::get().unwrap())?
        );
        return Ok(());
    }

    let key = k.unwrap();
    if let Some(set) = s {
        configuration::set(key, set).unwrap();
    }

    println!("{} = {}", key, configuration::get_value_as_str(key)?);

    Ok(())
}

fn init(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let repository = Repository::new(get_repository_path(&values)?, true, None)?;
    println!(
        "Initialized repository: {}",
        repository.workdir().unwrap().display()
    );

    Ok(())
}

fn addkey(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let mut repository = Repository::new(get_repository_path(&values)?, false, None)?;
    repository.add_key(None)?;

    Ok(())
}

fn rmkey(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let mut repository = Repository::new(get_repository_path(&values)?, false, None)?;
    repository.remove_key(None)?;

    Ok(())
}

fn ls(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let repository = Repository::new(get_repository_path(&values)?, false, None)?;
    let path = repository.path(values.get_positional_single("path"))?;
    for entry in &repository.list(Some(&path))? {
        println!("{}", entry.to_str().unwrap());
    }

    Ok(())
}

fn print_stored_data(retrieved: SensitiveData, force_binary: bool) -> Result<()> {
    let tty = isatty::stdout_isatty();
    let display: Optional<String> = retrieved.display(force_binary, tty);
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

fn get(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let repository = Repository::new(get_repository_path(&values)?, false, None)?;
    let path = repository.path(values.get_positional_single("path"))?;
    let force_binary = values.get_boolean("binary");

    let retrieved = repository.read_decrypt(&path)?;

    match () {
        #[cfg(feature = "clipboard")]
        () => if values.get_boolean("clipboard") {
            pwm_lib::util::clipboard::set_contents(retrieved, force_binary)?;
        } else {
            print_stored_data(retrieved, force_binary)?;
        },

        #[cfg(not(feature = "clipboard"))]
        () => {
            print_stored_data(retrieved, force_binary)?;
        },
    }

    Ok(())
}

fn set(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let repository = Repository::new(get_repository_path(&values)?, false, None)?;
    let path = repository.path(values.get_positional_single("path"))?;
    let key_file = values.get_single("key_file");
    let multiline = values.get_boolean("multiline");

    if key_file.is_some() && multiline {
        bail!("The 'key_file' and 'multiline' options are mutually exclusive.");
    }

    if let Some(key_file) = key_file {
        // The user wants to set the password using a key file.
        let mut key_file = File::open(key_file)?;
        repository.write_encrypt(&path, SensitiveData::from_file(&mut key_file)?)?;
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

fn rm(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let repository = Repository::new(get_repository_path(&values)?, false, None)?;
    let path = repository.path(values.get_positional_single("path"))?;
    repository.remove(&path)?;
    Ok(())
}

fn generate(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let mut charsets: Vec<pwgen::CharacterSet> = Vec::new();
    if !values.get_boolean("exclude_letters") {
        charsets.push(pwgen::CharacterSet::Letters);
    }
    if !values.get_boolean("exclude_numbers") {
        charsets.push(pwgen::CharacterSet::Numbers);
    }
    if values.get_boolean("include_symbols") {
        charsets.push(pwgen::CharacterSet::Symbols);
    }

    let length: usize = values.get_required_parsed("password_length")?;
    let custom_exclude: Vec<char> = values
        .get_single("custom_exclude")
        .map_or(vec![], |x| x.chars().collect());

    println!(
        "{}",
        pwgen::generate_password(length, charsets.as_slice(), custom_exclude.as_slice())?
            .display(false, false)
            .unwrap()
    );

    Ok(())
}

fn export(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let repository = Repository::new(get_repository_path(&values)?, false, None)?;
    println!("{}", export_serialize(&repository)?);
    Ok(())
}

fn import(values: Values) -> Result<()> {
    use std::io::Read;

    let _handle = init_pwm()?;

    let repository = Repository::new(get_repository_path(&values)?, false, None)?;

    let input_path = values.get_required("input");
    let mut input = String::new();
    let mut f = File::open(input_path)?;
    f.read_to_string(&mut input)?;

    import_deserialize(&repository, input.as_str())?;

    Ok(())
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    bdrck::logging::init(None, None, false);

    main_impl_multiple_commands(vec![
        Command::new(
            "config",
            "Get or set a configuration value",
            Specs::new(vec![
                Spec::optional("set", "The new value to set the key to", Some('s')),
                Spec::optional("key", "The specific key to get or set", Some('k')),
            ]).unwrap(),
            Box::new(config)),
        Command::new(
            "init",
            "Initialize a new pwm repository",
            Specs::new(vec![
                Spec::optional("repository", "The path to the repository to initialize", Some('r')),
            ]).unwrap(),
            Box::new(init)),
        Command::new(
            "addkey",
            "Add a new master key to an existing repository",
            Specs::new(vec![
                Spec::optional(
                    "repository", "The path to the repository to add a key to", Some('r')),
            ]).unwrap(),
            Box::new(addkey)),
        Command::new(
            "rmkey",
            "Remove an existing master key from an existing repository",
            Specs::new(vec![
                Spec::optional(
                    "repository", "The path to the repository to remove a key from", Some('r')),
            ]).unwrap(),
            Box::new(rmkey)),
        Command::new(
            "ls",
            "List passwords stored in a pwm repository",
            Specs::new(vec![
                Spec::optional(
                    "repository", "The path to the repository to list the contents of", Some('r')),
                Spec::positional(
                    "path",
                    "The path to list, relative to the repository's root",
                    Some(&[""]),
                    false,
                ).unwrap(),
            ]).unwrap(),
            Box::new(ls)),
        Command::new(
            "get",
            "Retrieve a password or key from a pwm repository",
            Specs::new(if cfg!(feature = "clipboard") {
                vec![
                    Spec::optional(
                        "repository",
                        "The path to the repository to retrieve the password or key from",
                        Some('r'),
                    ),
                    Spec::boolean(
                        "binary", "Treat the retrieved password or key as binary", Some('b')),
                    Spec::boolean(
                        "clipboard", "Copy the password or key to the clipboard", Some('c')),
                    Spec::positional(
                        "path",
                        "The path to retrieve, relative to the repository's root",
                        None,
                        false,
                    ).unwrap(),
                ]
            } else {
                vec![
                    Spec::optional(
                        "repository",
                        "The path to the repository to retrieve the password or key from",
                        Some('r'),
                    ),
                    Spec::boolean(
                        "binary", "Treat the retrieved password or key as binary", Some('b')),
                    Spec::positional(
                        "path",
                        "The path to retrieve, relative to the repository's root",
                        None,
                        false,
                    ).unwrap(),
                ]
            }).unwrap(),
            Box::new(get)),
        Command::new(
            "set",
            "Store a password or key in a pwm repository",
            Specs::new(vec![
                Spec::optional("repository", "The path to the repository to modify", Some('r')),
                Spec::optional("key_file", "Store a key file instead of a password", Some('k')),
                Spec::boolean(
                    "multiline", "Read multiple lines of input data, until 'EOF'", Some('m')),
                Spec::positional(
                    "path",
                    "The path to set, relative to the repository's root",
                    None,
                    false,
                ).unwrap(),
            ]).unwrap(),
            Box::new(set)),
        Command::new(
            "rm",
            "Remove a password or key from a pwm repository",
            Specs::new(vec![
                Spec::optional("repository", "The path to the repository to modify", Some('r')),
                Spec::positional(
                    "path",
                    "The path to remove, relative to the repository's root",
                    None,
                    false,
                ).unwrap(),
            ]).unwrap(),
            Box::new(rm)),
        Command::new(
            "generate",
            "Generate a random password",
            Specs::new(vec![
                Spec::required(
                    "password_length",
                    "The length of the password to generate",
                    Some('l'),
                    Some(pwgen::RECOMMENDED_MINIMUM_PASSWORD_LENGTH.to_string().as_str()),
                ),
                Spec::boolean("exclude_letters", "Exclude letters from the password", Some('A')),
                Spec::boolean("exclude_numbers", "Exclude numbers from the password", Some('N')),
                Spec::boolean("include_symbols", "Include symbols in the password", Some('s')),
                Spec::optional("custom_exclude", "Exclute a custom set of characters", Some('x')),
            ]).unwrap(),
            Box::new(generate)),
        Command::new(
            "export",
            "Export all stored passwords as plaintext JSON for backup purposes",
            Specs::new(vec![
                Spec::optional(
                    "repository", "The path to the repository to export from", Some('r')),
            ]).unwrap(),
            Box::new(export)),
        Command::new(
            "import",
            "Import stored passwords previously 'export'ed",
            Specs::new(vec![
                Spec::optional(
                    "repository", "The path to the repository to import into", Some('r')),
                Spec::required("input", "The input file to import from", Some('i'), None),
            ]).unwrap(),
            Box::new(import)),
    ]);
}
