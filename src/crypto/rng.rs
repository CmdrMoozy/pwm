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

use bdrck::crypto::util::randombytes_into;
use byteorder::{LittleEndian, ReadBytesExt};
use rand::{self, RngCore};
use std::io::Cursor;

/// This structure implements the `Rng` trait from the `rand` crate using
/// `bdrck`'s `randombytes_into` function. This implies that this random
/// number generator is both thread safe, and cryptographically secure (e.g.
/// suitable for generating key material or passwords).
pub struct Generator;

impl RngCore for Generator {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0_u8; std::mem::size_of::<u64>()];
        randombytes_into(&mut buf);
        let mut rdr = Cursor::new(&buf);
        rdr.read_u64::<LittleEndian>().unwrap()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        randombytes_into(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> ::std::result::Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}
