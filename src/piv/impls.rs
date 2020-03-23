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
use crate::crypto::pwgen;
use crate::error::*;
use crate::piv::util::{prompt_for_device, PivKeyAssociation};
use crate::repository::Repository;
use bdrck::crypto::key::AbstractKey;
use flaggy::*;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use yubirs::piv;
use yubirs::piv::id::{Algorithm, Key, PinPolicy, TouchPolicy};

fn addpiv_impl<RP: AsRef<Path>>(
    repository_path: RP,
    reader: &str,
    serial: u32,
    slot: Key,
    public_key: piv::pkey::PublicKey,
) -> Result<()> {
    // Add the key to the repository.
    let mut repository = Repository::new(repository_path.as_ref(), false, None)?;
    let public_key_pem = public_key.format(piv::pkey::Format::Pem)?;
    let key: piv::key::Key<piv::hal::PcscHardware> =
        piv::key::Key::new(Some(reader), None, slot, public_key)?;
    repository.add_key(&key)?;

    // Also add the key to our configuration.
    let mut configuration = repository.get_crypto_configuration();
    configuration.add_piv_key(PivKeyAssociation {
        reader: reader.to_owned(),
        serial: serial,
        wrapping_key_digest: key.get_digest(),
        slot: slot,
        public_key_pem: public_key_pem,
    });
    repository.set_crypto_configuration(configuration);

    Ok(())
}

#[command_callback]
pub(crate) fn setuppiv(
    repository: Option<PathBuf>,
    slot: Key,
    algorithm: Algorithm,
    pin_policy: PinPolicy,
    touch_policy: TouchPolicy,
    public_key: Option<PathBuf>,
) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;

    // This is a very destructive operation; confirm with the user first before
    // proceeding.
    if !bdrck::cli::continue_confirmation(
        bdrck::cli::Stream::Stderr,
        "WARNING: This will reset all PIV device data (certificates, ...) to factory defaults. ",
    )? {
        return Ok(());
    }

    let (reader, serial) = prompt_for_device(None, None)?;
    let generated_public_key: Option<piv::pkey::PublicKey>;

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
        let pubkey = handle.generate(
            Some(new_mgm_key.as_str()),
            slot,
            algorithm,
            pin_policy,
            touch_policy,
        )?;

        // Write the public key to a file.
        if let Some(path) = public_key {
            let public_key_data = pubkey.format(piv::pkey::Format::Pem)?;
            let mut public_key_file = File::create(&path)?;
            public_key_file.write_all(&public_key_data)?;
        }

        generated_public_key = Some(pubkey);
    }

    // Actually add the new PIV device.
    addpiv_impl(
        &repository,
        &reader,
        serial,
        slot,
        generated_public_key.unwrap(),
    )
}

#[command_callback]
pub(crate) fn addpiv(repository: Option<PathBuf>, slot: Key, public_key: PathBuf) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let repository = get_repository_path(repository)?;
    let (reader, serial) = prompt_for_device(None, None)?;
    let public_key = piv::pkey::PublicKey::from_pem_file(&public_key)?;
    addpiv_impl(&repository, &reader, serial, slot, public_key)
}

#[command_callback]
pub(crate) fn rmpiv(
    _repository: Option<PathBuf>,
    _reader: Option<String>,
    _serial: Option<u32>,
) -> Result<()> {
    Ok(())
}
