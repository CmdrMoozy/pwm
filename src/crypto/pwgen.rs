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

use crate::crypto::rng::Generator;
use crate::error::*;
use crate::secret::Secret;
use lazy_static::lazy_static;
use rand::{Rng, RngCore};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CharacterSet {
    Letters,
    Numbers,
    Symbols,
}

lazy_static! {
    static ref CHARACTER_SET: HashMap<CharacterSet, Vec<u8>> = {
        let mut m = HashMap::new();

        for c in u8::MIN..u8::MAX {
            if let Some(cc) = char::from_u32(c as u32) {
                let key = if !cc.is_ascii() {
                    continue;
                } else if cc.is_ascii_alphabetic() {
                    CharacterSet::Letters
                } else if cc.is_ascii_digit() {
                    CharacterSet::Numbers
                } else if cc.is_ascii_graphic() {
                    // is_ascii_graphic, in this else block, means printable non-whitespace ASCII,
                    // except alphanumeric characters.
                    CharacterSet::Symbols
                } else {
                    continue;
                };

                let set = m.entry(key).or_insert_with(Vec::new);
                set.push(c);
            }
        }

        m
    };
}

pub const RECOMMENDED_MINIMUM_PASSWORD_LENGTH: usize = 16;

pub fn generate_password(
    length: usize,
    charsets: &[CharacterSet],
    exclude: &[char],
) -> Result<Secret> {
    if length == 0 {
        bail!("refusing to generate a password of length 0");
    }

    let exclude: HashSet<u8> = exclude
        .iter()
        .filter_map(|c| {
            if c.is_ascii() {
                let mut buf = [0; 1];
                // Panics if c takes > 1 byte, but since we checked is_ascii() this should never happen.
                c.encode_utf8(&mut buf);
                Some(buf[0])
            } else {
                None
            }
        })
        .collect();

    let chars: Vec<u8> = charsets
        .iter()
        .flat_map(|cs| CHARACTER_SET.get(cs).unwrap().iter())
        .filter(|c| !exclude.contains(c))
        .cloned()
        .collect();

    if chars.is_empty() {
        bail!("cannot generate passwords from an empty character set");
    }

    let mut generator = Generator;
    let mut result = Secret::with_len(length);

    for i in 0..length {
        result.as_mut_slice()[i] = chars[generator.gen_range(0..chars.len())];
    }

    Ok(result)
}

pub fn generate_hex(byte_length: usize) -> String {
    let mut generator = Generator;
    let mut bytes = vec![0_u8; byte_length];
    generator.fill_bytes(&mut bytes);
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .concat()
}
