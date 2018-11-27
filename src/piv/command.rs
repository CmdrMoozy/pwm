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

use bdrck::flags::Values;
use crypto::pwgen;
use error::*;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use yubirs::piv;

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

pub fn setuppiv(values: Values) -> Result<()> {
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

    // Actually add the new PIV device.
    addpiv_impl(
        &reader,
        values.get_required_parsed("slot")?,
        &public_key_path,
    )
}

fn addpiv_impl<P: AsRef<Path>>(_reader: &str, _slot: piv::id::Key, _public_key: P) -> Result<()> {
    Ok(())
}

pub fn addpiv(values: Values) -> Result<()> {
    let reader = prompt_for_reader()?;
    let slot: piv::id::Key = values.get_required_parsed("slot")?;
    let public_key: PathBuf = values.get_required_as("public_key");

    addpiv_impl(&reader, slot, &public_key)
}
