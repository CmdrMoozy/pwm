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

use byteorder::{LittleEndian, ReadBytesExt};
use rand::Rng;
use sodiumoxide::randombytes::randombytes;
use std::io::Cursor;

/// This structure implements the `Rng` trait from the `rand` crate using
/// `sodiumoxide`'s `randombytes` function. This implies that this random
/// number generator is both thread safe, and cryptographically secure (e.g.
/// suitable for generating key material or passwords).
pub struct Generator;

impl Rng for Generator {
    fn next_u32(&mut self) -> u32 {
        let bytes = randombytes(4);
        let mut rdr = Cursor::new(bytes);
        rdr.read_u32::<LittleEndian>().unwrap()
    }
}
