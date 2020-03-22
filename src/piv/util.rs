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
use crate::error::*;
use bdrck::crypto::key::{AbstractKey, Digest};
use log::warn;
use serde_derive::{Deserialize, Serialize};
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
    let mut handle: piv::Handle<piv::PcscHardware> = piv::Handle::new()?;
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

/// Try to find a PIV device key which can be used for unwrapping our key store.
/// This can return an error if something unexpected happens, or `Ok(None)` if
/// we don't manage to find any key and nothing catastrophic goes wrong.
pub(crate) fn find_master_key(
    crypto_config: &Configuration,
) -> Result<Option<Box<dyn AbstractKey>>> {
    for (reader, serial) in list_piv_devices()? {
        for assoc in crypto_config.get_piv_keys() {
            if serial == assoc.serial {
                // It's weird if the reader name changed, but we'll take the
                // serial number as being authoritative and continue anyway
                // (after logging a warning).
                if reader != assoc.reader {
                    warn!(
                        "Found matching PIV device, with reader mismatch (expected '{}', got '{}'",
                        assoc.reader, reader
                    );
                }

                let key: Option<Box<dyn AbstractKey>> =
                    match piv::key::Key::<piv::PcscHardware>::new(
                        Some(&reader),
                        /*pin=*/ None,
                        assoc.slot,
                        assoc.public_key_pem.as_slice(),
                    ) {
                        Ok(k) => Some(Box::new(k)),
                        Err(e) => {
                            warn!("Failed to get master key from recognized PIV device '{}' (serial # {}): {}", reader, assoc.serial, e);
                            None
                        }
                    };

                if let Some(k) = key {
                    return Ok(Some(k));
                }
            }
        }
    }

    Ok(None)
}
