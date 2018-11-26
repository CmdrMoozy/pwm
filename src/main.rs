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

#![deny(
    anonymous_parameters,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces
)]
#![warn(bare_trait_objects, unreachable_pub, unused_qualifications)]

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

extern crate bdrck;
use bdrck::flags::*;

#[macro_use]
extern crate failure;

extern crate pwm_lib;
use pwm_lib::configuration;
use pwm_lib::crypto::pwgen;
use pwm_lib::error::*;
use pwm_lib::repository::serde::{export_serialize, import_deserialize};
use pwm_lib::repository::Repository;
use pwm_lib::util::data::{end_user_display, load_file, SecretSlice};
use pwm_lib::util::{multiline_password_prompt, password_prompt};

extern crate serde_json;

#[cfg(feature = "yubikey")]
extern crate yubirs;

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
        None => return Err(Error::InvalidArgument(format_err!("No repository path specified. Try the 'repository' command option, or setting the 'default_repository' configuration key."))),
    }
}

fn config(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let k = values.get_single("key");
    let s = values.get_single("set");
    if k.is_none() {
        if s.is_some() {
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

#[cfg(feature = "yubikey")]
fn prompt_for_reader() -> Result<String> {
    use yubirs::piv;

    let handle: piv::Handle<piv::PcscHardware> = piv::Handle::new()?;
    let mut readers = handle.list_readers()?;
    Ok(match readers.len() {
        0 => {
            return Err(Error::InvalidArgument(format_err!(
                "No PIV devices found on this system"
            )))
        }
        1 => readers.pop().unwrap(),
        _ => {
            let mut stderr = io::stderr();
            let mut i: usize = 1;
            for reader in &readers {
                write!(stderr, "{}: {}\n", i, reader)?;
                i += 1;
            }
            stderr.flush()?;

            let mut reader: Option<String>;
            let prompt = format!("Which PIV device to set up? [1..{}] ", readers.len());
            loop {
                let choice = bdrck::cli::prompt_for_string(
                    bdrck::cli::Stream::Stderr,
                    prompt.as_str(),
                    false,
                )?;
                match choice.parse::<usize>() {
                    Err(_) => {
                        write!(stderr, "Invalid number '{}'.\n", choice)?;
                        stderr.flush()?;
                    }
                    Ok(idx) => {
                        if idx < 1 || idx > readers.len() {
                            write!(stderr, "Invalid choice '{}'.\n", idx)?;
                            stderr.flush()?;
                        } else {
                            reader = Some(readers.get(idx - 1).unwrap().clone());
                            break;
                        }
                    }
                };
            }
            reader.unwrap()
        }
    })
}

#[cfg(feature = "yubikey")]
fn setuppiv(values: Values) -> Result<()> {
    use yubirs::piv;

    let _handle = init_pwm()?;

    // This is a very destructive operation; confirm with the user first before
    // proceeding.
    if !bdrck::cli::continue_confirmation(
        bdrck::cli::Stream::Stderr,
        "This will reset all PIV device data (certificates, ...) to factory defaults. ",
    )? {
        return Ok(());
    }

    let reader = prompt_for_reader()?;

    let mut handle: piv::Handle<piv::PcscHardware> = piv::Handle::new()?;
    handle.connect(Some(reader.as_str()))?;
    handle.force_reset()?;

    // Generate the various new access keys and configure the device.
    let new_pin = pwgen::generate_hex(3);
    let new_puk = pwgen::generate_hex(4);
    let new_mgm_key = pwgen::generate_hex(24);
    println!("Your new PIN is: {}", new_pin);
    println!("Your new PUK is: {}", new_puk);
    println!("Your new management key is: {}", new_mgm_key);
    handle.change_pin(Some(piv::DEFAULT_PIN), Some(new_pin.as_str()))?;
    handle.change_puk(Some(piv::DEFAULT_PUK), Some(new_puk.as_str()))?;
    handle.set_management_key(
        Some(piv::DEFAULT_MGM_KEY),
        Some(new_mgm_key.as_str()),
        false,
    )?;

    // Generate a CHUID and CCC, each of which are required by some OSes before
    // they will fully recognize the PIV hardware.
    handle.set_chuid(Some(new_mgm_key.as_str()))?;
    handle.set_ccc(Some(new_mgm_key.as_str()))?;

    // Generate the certificate pair which will be used to wrap the
    // repository's master key.
    let public_key = handle.generate(
        Some(new_mgm_key.as_str()),
        values.get_required_parsed("slot")?,
        values.get_required_parsed("algorithm")?,
        values.get_required_parsed("pin_policy")?,
        values.get_required_parsed("touch_policy")?,
    )?;
    let public_key_data = public_key.format(piv::pkey::Format::Pem)?;

    // Write the public key to a file.
    let public_key_path: PathBuf = values.get_required_as("public_key");
    {
        let mut public_key_file = File::create(public_key_path.as_path())?;
        public_key_file.write_all(&public_key_data)?;
    }

    Ok(())
}

#[cfg(feature = "yubikey")]
fn addpiv(_values: Values) -> Result<()> {
    let _handle = init_pwm()?;
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

fn get(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let repository = Repository::new(get_repository_path(&values)?, false, None)?;
    let path = repository.path(values.get_positional_single("path"))?;
    let force_binary = values.get_boolean("binary");

    let retrieved = repository.read_decrypt(&path)?;

    match () {
        #[cfg(feature = "clipboard")]
        () => if values.get_boolean("clipboard") {
            pwm_lib::util::clipboard::set_contents(&retrieved, force_binary)?;
        } else {
            print_stored_data(&retrieved, force_binary)?;
        },

        #[cfg(not(feature = "clipboard"))]
        () => {
            print_stored_data(&retrieved, force_binary)?;
        }
    }

    Ok(())
}

fn set(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let mut repository = Repository::new(get_repository_path(&values)?, false, None)?;
    let path = repository.path(values.get_positional_single("path"))?;
    let key_file = values.get_single("key_file");
    let multiline = values.get_boolean("multiline");

    if key_file.is_some() && multiline {
        return Err(Error::InvalidArgument(format_err!(
            "The 'key_file' and 'multiline' options are mutually exclusive."
        )));
    }

    if let Some(key_file) = key_file {
        // The user wants to set the password using a key file.
        repository.write_encrypt(&path, load_file(key_file)?)?;
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

    let mut repository = Repository::new(get_repository_path(&values)?, false, None)?;
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
        end_user_display(
            pwgen::generate_password(length, charsets.as_slice(), custom_exclude.as_slice())?
                .as_slice(),
            false,
            false
        ).unwrap()
    );

    Ok(())
}

fn export(values: Values) -> Result<()> {
    let _handle = init_pwm()?;

    let mut repository = Repository::new(get_repository_path(&values)?, false, None)?;
    println!("{}", export_serialize(&mut repository)?);
    Ok(())
}

fn import(values: Values) -> Result<()> {
    use std::io::Read;

    let _handle = init_pwm()?;

    let mut repository = Repository::new(get_repository_path(&values)?, false, None)?;

    let input_path = values.get_required("input");
    let mut input = String::new();
    let mut f = File::open(input_path)?;
    f.read_to_string(&mut input)?;

    import_deserialize(&mut repository, input.as_str())?;

    Ok(())
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let debug: bool = cfg!(debug_assertions);
    bdrck::logging::init(
        bdrck::logging::OptionsBuilder::new()
            .set_filters(match debug {
                false => "warn".parse().unwrap(),
                true => "debug".parse().unwrap(),
            })
            .set_panic_on_output_failure(debug)
            .set_always_flush(true)
            .build()
            .unwrap(),
    );

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
        #[cfg(feature = "yubikey")]
        Command::new(
            "setuppiv",
            "Set up a PIV device and add it to an existing repository",
            Specs::new(vec![
                Spec::optional("repository", "The path to the repository to remove a key from", Some('r')),
                Spec::required(
                    "slot", "The slot containing the certificate to use", Some('s'),
                    Some(&::yubirs::piv::id::Key::KeyManagement.to_string())),
                Spec::required(
                    "algorithm", "The key algorithm to use", Some('a'),
                    Some(&::yubirs::piv::id::Algorithm::Rsa2048.to_string())),
                Spec::required(
                    "pin_policy", "The PIN verification policy to use for this key", None,
                    Some(&::yubirs::piv::id::PinPolicy::Default.to_string())),
                Spec::required(
                    "touch_policy", "The touch policy to use for this key", None,
                    Some(&::yubirs::piv::id::TouchPolicy::Default.to_string())),
                Spec::required("public_key", "The path to write the public key to", Some('p'), None),
            ]).unwrap(),
            Box::new(setuppiv)),
        #[cfg(feature = "yubikey")]
        Command::new(
            "addpiv",
            "Add an already set up PIV device to an existing repository",
            Specs::new(vec![
                Spec::optional("repository", "The path to the repository to remove a key from", Some('r')),
                Spec::required(
                    "slot", "The slot containing the certificate to use", Some('s'),
                    Some(&::yubirs::piv::id::Key::KeyManagement.to_string())),
                Spec::required("public_key", "The path to write the public key to", Some('p'), None),
            ]).unwrap(),
            Box::new(addpiv)),
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
