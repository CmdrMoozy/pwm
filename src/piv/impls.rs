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
use crate::piv::{Configuration, KeyConfiguration};
use crate::repository::Repository;
use failure::format_err;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use yubirs::piv;
use yubirs::piv::id::{Algorithm, Key, PinPolicy, TouchPolicy};

fn prompt_for_reader() -> Result<String> {
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

            let reader: Option<String>;
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

fn addpiv_impl<RP: AsRef<Path>, KP: AsRef<Path>>(
    repository_path: RP,
    reader: &str,
    slot: Key,
    public_key_path: KP,
) -> Result<()> {
    // Add the key to the repository.
    let mut repository = Repository::new(repository_path.as_ref(), false, None)?;
    let key: piv::key::Key<piv::hal::PcscHardware> =
        piv::key::Key::new(Some(reader), None, slot, public_key_path.as_ref())?;
    repository.add_key(&key)?;

    // Also add the key to our configuration.
    configuration::instance_apply_mut(|config: &mut configuration::Configuration| -> Result<()> {
        let mut piv_config = config
            .piv
            .as_ref()
            .cloned()
            .unwrap_or_else(Configuration::default);
        piv_config.keys.push(KeyConfiguration {
            reader: Some(reader.to_owned()),
            slot: slot,
            public_key: fs::canonicalize(public_key_path.as_ref())?,
        });
        config.piv = Some(piv_config);
        Ok(())
    })?;

    Ok(())
}

pub(crate) fn setuppiv<RP: AsRef<Path>, KP: AsRef<Path>>(
    repository_path: RP,
    slot: Key,
    algorithm: Algorithm,
    pin_policy: PinPolicy,
    touch_policy: TouchPolicy,
    public_key_path: KP,
) -> Result<()> {
    // This is a very destructive operation; confirm with the user first before
    // proceeding.
    if !bdrck::cli::continue_confirmation(
        bdrck::cli::Stream::Stderr,
        "This will reset all PIV device data (certificates, ...) to factory defaults. ",
    )? {
        return Ok(());
    }

    let reader = prompt_for_reader()?;

    {
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
            slot,
            algorithm,
            pin_policy,
            touch_policy,
        )?;
        let public_key_data = public_key.format(piv::pkey::Format::Pem)?;

        // Write the public key to a file.
        let mut public_key_file = File::create(public_key_path.as_ref())?;
        public_key_file.write_all(&public_key_data)?;
    }

    // Actually add the new PIV device.
    addpiv_impl(repository_path, &reader, slot, public_key_path)
}

pub(crate) fn addpiv<RP: AsRef<Path>, KP: AsRef<Path>>(
    repository_path: RP,
    slot: Key,
    public_key_path: KP,
) -> Result<()> {
    let reader = prompt_for_reader()?;
    addpiv_impl(repository_path, &reader, slot, public_key_path)
}
