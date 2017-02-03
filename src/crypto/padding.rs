// pwm - A simple password manager for Linux.
// Copyright (C) 2015  Axel Rasmussen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use error::Result;
use sodiumoxide::randombytes::randombytes;
use std::io::Cursor;
use std::mem;
use util::data::SensitiveData;

const PAD_BLOCK_SIZE_BYTES: usize = 1024;

fn get_padded_size(original_size: usize) -> usize {
    let padded_size = original_size + mem::size_of::<u64>();
    let blocks = (padded_size / PAD_BLOCK_SIZE_BYTES) +
                 if padded_size % PAD_BLOCK_SIZE_BYTES == 0 {
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
    original_size_encoded.write_u64::<BigEndian>(original_size as u64).unwrap();

    data.concat(SensitiveData::from(randombytes(padding_bytes)))
        .concat(SensitiveData::from(original_size_encoded))
}

pub fn unpad(data: SensitiveData) -> Result<SensitiveData> {
    let original_size = try!(read_original_size(&data));
    Ok(data.truncate(original_size))
}
