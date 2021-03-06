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

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use error::Result;
use sodiumoxide::randombytes::randombytes;
use std::io::Cursor;
use std::mem;
use util::data::SensitiveData;

const PAD_BLOCK_SIZE_BYTES: usize = 1024;

fn get_padded_size(original_size: usize) -> usize {
    let padded_size = original_size + mem::size_of::<u64>();
    let blocks = (padded_size / PAD_BLOCK_SIZE_BYTES)
        + if padded_size % PAD_BLOCK_SIZE_BYTES == 0 {
            0
        } else {
            1
        };
    blocks * PAD_BLOCK_SIZE_BYTES
}

fn read_original_size(data: &SensitiveData) -> Result<usize> {
    if data.len() < mem::size_of::<u64>() {
        bail!("Cannot unpad data with invalid length");
    }

    let original_size_encoded: &[u8] = &data[data.len() - mem::size_of::<u64>()..];
    let mut reader = Cursor::new(original_size_encoded);
    Ok(reader.read_u64::<BigEndian>().unwrap() as usize)
}

pub fn pad(data: SensitiveData) -> SensitiveData {
    let original_size: usize = data.len();
    let padded_size = get_padded_size(original_size);
    let padding_bytes = padded_size - original_size - mem::size_of::<u64>();

    let mut original_size_encoded: Vec<u8> = vec![];
    original_size_encoded
        .write_u64::<BigEndian>(original_size as u64)
        .unwrap();

    data.concat(SensitiveData::from(randombytes(padding_bytes)))
        .concat(SensitiveData::from(original_size_encoded))
}

pub fn unpad(data: SensitiveData) -> Result<SensitiveData> {
    let original_size = read_original_size(&data)?;
    Ok(data.truncate(original_size))
}
