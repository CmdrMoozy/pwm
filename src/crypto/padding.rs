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
use crate::secret::Secret;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::mem;

const PAD_BLOCK_SIZE_BYTES: usize = 1024;
const METADATA_BYTES: usize = mem::size_of::<u64>();

/// Returns the number of bytes needed to represent `original_size` bytes, after
/// padding and metadata have been added to it. This is the total size of the
/// data after calling `pad` on it.
fn get_padded_size(original_size: usize) -> usize {
    let padded_size = original_size + METADATA_BYTES;
    let blocks = (padded_size / PAD_BLOCK_SIZE_BYTES)
        + if padded_size % PAD_BLOCK_SIZE_BYTES == 0 {
            0
        } else {
            1
        };
    blocks * PAD_BLOCK_SIZE_BYTES
}

fn read_original_size(data: &Secret) -> Result<usize> {
    if data.len() % PAD_BLOCK_SIZE_BYTES != 0 {
        return Err(Error::InvalidArgument(
            "cannot unpad data which wasn't previously padded - bad length".to_string(),
        ));
    }

    if data.len() < METADATA_BYTES {
        return Err(Error::InvalidArgument(
            "cannot unpad data with invalid length".to_string(),
        ));
    }

    let original_size_encoded = &data[data.len() - METADATA_BYTES..];
    let mut reader = Cursor::new(original_size_encoded);
    Ok(reader.read_u64::<BigEndian>().unwrap() as usize)
}

pub fn pad(data: &mut Secret) {
    let original_size: usize = data.len();
    let padded_size = get_padded_size(original_size);

    if cfg!(debug_assertions) {
        // For debug builds, just pad with zeros so we generate deterministic output.
        data.resize(padded_size - METADATA_BYTES, 0);
    } else {
        // For release builds, pad with random bytes. This probably doesn't increase security? But
        // it seems like it might prevent some edge case leaks, so it can't hurt.
        data.append(&mut sodiumoxide::randombytes::randombytes(
            padded_size - original_size - METADATA_BYTES,
        ));
    }

    data.write_u64::<BigEndian>(original_size as u64).unwrap();
}

pub fn unpad(data: &mut Secret) -> Result<()> {
    let original_size = read_original_size(&data)?;
    data.truncate(original_size);
    Ok(())
}
