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

use crate::error::*;
use bdrck::crypto::key::Digest;
use serde_derive::{Deserialize, Serialize};
use yubirs::piv;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PivKeyAssociation {
    pub(crate) reader: String,
    pub(crate) chuid: Vec<u8>,

    pub(crate) wrapping_key_digest: Digest,

    pub(crate) slot: piv::id::Key,
    pub(crate) public_key_pem: Vec<u8>,
}

/// List all of the PIV devices currently attached to the system. Returns a
/// `Vec` of tuples of (reader name, CHUID data). A unique CHUID must uniquely
/// identify not only a device, but also *its contents* (OSes cache contents
/// aggressively, so if they change the user must update the CHUID).
pub(crate) fn list_piv_devices() -> Result<Vec<(String, Vec<u8>)>> {
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

        let chuid = handle.read_object(piv::id::Object::Chuid)?;
        devices.push((reader, chuid));
    }

    Ok(devices)
}
