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

use crate::crypto::configuration::Configuration;
use crate::crypto::key::PwmKey;
use crate::error::*;
use bdrck::crypto::key::{AbstractKey, Digest};
use log::warn;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use yubirs::piv;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PivKeyAssociation {
    pub(crate) reader: String,
    pub(crate) serial: u32,

    pub(crate) wrapping_key_digest: Digest,

    pub(crate) slot: piv::id::Key,
    pub(crate) public_key_pem: Vec<u8>,
}

/// List all of the PIV devices currently attached to the system. Returns a
/// `Vec` of tuples of (reader name, CHUID data). A unique CHUID must uniquely
/// identify not only a device, but also *its contents* (OSes cache contents
/// aggressively, so if they change the user must update the CHUID).
pub(crate) fn list_piv_devices() -> Result<Vec<(String, u32)>> {
    let mut handle: piv::Handle<piv::PcscHardware> = match piv::Handle::new() {
        Ok(h) => h,
        /*
         * Constructing a handle might fail for legitimate reasons, e.g. if there are no smartcards
         * attached to the system. Don't fail completely in that case, just log the error and
         * return a list of no readers.
         */
        Err(e) => {
            warn!(
                "failed to establish PCSC context ({}); no smart cards attached?",
                e
            );
            return Ok(vec![]);
        }
    };
    let mut connected = false;

    let readers = handle.list_readers()?;
    let mut devices = Vec::with_capacity(readers.len());
    for reader in readers {
        // TODO: This sucks, Handle should take care of this for us.
        if connected {
            handle.disconnect();
        }
        handle.connect(Some(&reader))?;
        connected = true;

        let serial = handle.get_serial()?;
        devices.push((reader, serial.0));
    }

    Ok(devices)
}

/// Prompts the user on stderr for a single device out of the given list of all
/// available PIV devices. The list is filtered to only devices matching
/// `reader` and/or `serial`, if provided.
pub(crate) fn prompt_for_device_from(
    reader: Option<&str>,
    serial: Option<u32>,
    devices: Vec<(String, u32)>,
) -> Result<(String, u32)> {
    let mut devices: Vec<(String, u32)> = devices
        .into_iter()
        .filter(|(r, s)| {
            if let Some(reader) = reader {
                if *r != reader {
                    return false;
                }
            }

            if let Some(serial) = serial {
                if *s != serial {
                    return false;
                }
            }

            true
        })
        .collect();

    if devices.is_empty() {
        return Err(Error::InvalidArgument(
            "no matching PIV devices found on this system".to_string(),
        ));
    }

    if devices.len() == 1 {
        return Ok(devices.into_iter().next().unwrap());
    }

    let mut stderr = io::stderr();
    for (i, (reader, serial)) in devices.iter().enumerate() {
        write!(stderr, "{}: {} (serial # {})\n", i, reader, serial)?;
    }
    stderr.flush()?;

    let selection: Option<(String, u32)>;
    let prompt = format!("Select the PIV device to use? [1..{}]", devices.len());
    loop {
        let i = bdrck::cli::prompt_for_string(
            bdrck::cli::Stream::Stdin,
            bdrck::cli::Stream::Stderr,
            prompt.as_str(),
            false,
        )?;
        let i = match i.parse::<usize>() {
            Err(_) => {
                write!(stderr, "Invalid number '{}'.\n", i)?;
                stderr.flush()?;
                continue;
            }
            Ok(i) => i,
        };
        if i < 1 || i > devices.len() {
            write!(stderr, "Invalid choice '{}'.\n", i)?;
            stderr.flush()?;
            continue;
        }

        selection = Some(devices.remove(i - 1));
        break;
    }

    Ok(selection.unwrap())
}

/// Prompt the user on stderr for a single device out of all devices returned by
/// `list_piv_devices`. The list is filtered to only devices matching `reader`
/// and/or `serial`, if provided.
pub(crate) fn prompt_for_device(
    reader: Option<&str>,
    serial: Option<u32>,
) -> Result<(String, u32)> {
    prompt_for_device_from(reader, serial, list_piv_devices()?)
}

/// Try to find a PIV device key which can be used for unwrapping our key store.
/// This can return an error if something unexpected happens, or `Ok(None)` if
/// we don't manage to find any key and nothing catastrophic goes wrong.
pub(crate) fn find_master_key(
    crypto_config: &Configuration,
) -> Result<Option<impl AbstractKey<Error = Error>>> {
    let devices = list_piv_devices()?;

    for assoc in crypto_config.get_piv_keys() {
        for (reader, serial) in devices.iter() {
            if *serial == assoc.serial {
                // It's weird if the reader name changed, but we'll take the
                // serial number as being authoritative and continue anyway
                // (after logging a warning).
                if *reader != assoc.reader {
                    warn!(
                        "Found matching PIV device, with reader mismatch (expected '{}', got '{}'",
                        assoc.reader, reader
                    );
                }

                // Confirm with the user that they wish to use this device.
                if !bdrck::cli::continue_confirmation(
                    bdrck::cli::Stream::Stdin,
                    bdrck::cli::Stream::Stderr,
                    &format!(
                        "Unlock repository using PIV device '{}' (#{})? ",
                        reader, serial
                    ),
                )? {
                    continue;
                }

                let key: Option<piv::key::Key<piv::PcscHardware>> = match piv::key::Key::<
                    piv::PcscHardware,
                >::new_from_read(
                    Some(&reader),
                    /*pin=*/ None,
                    assoc.slot,
                    assoc.public_key_pem.as_slice(),
                ) {
                    Ok(k) => Some(k),
                    Err(e) => {
                        warn!("Failed to get master key from recognized PIV device '{}' (serial # {}): {}", reader, assoc.serial, e);
                        None
                    }
                };

                if let Some(k) = key {
                    return Ok(Some(PwmKey::from(k)));
                }
            }
        }
    }

    Ok(None)
}
