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

use crypto::rng::Generator;
use error::*;
use rand::{Rng, RngCore};
use std::collections::{HashMap, HashSet};
use util::data::Secret;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CharacterSet {
    Letters,
    Numbers,
    Symbols,
}

lazy_static! {
    static ref CHARACTER_SET: HashMap<CharacterSet, Vec<char>> = {
        let mut m = HashMap::new();
        m.insert(
            CharacterSet::Letters,
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .collect(),
        );
        m.insert(CharacterSet::Numbers, "0123456789".chars().collect());
        m.insert(
            CharacterSet::Symbols,
            "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect(),
        );
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
        return Err(Error::InvalidArgument(format_err!(
            "Refusing to generate a password of length 0"
        )));
    }

    let exclude: HashSet<char> = exclude.iter().cloned().collect();
    let chars: Vec<char> = charsets
        .iter()
        .flat_map(|cs| CHARACTER_SET.get(cs).unwrap().iter())
        .filter(|c| !exclude.contains(c))
        .cloned()
        .collect();
    if chars.is_empty() {
        return Err(Error::InvalidArgument(format_err!(
            "Cannot generate passwords from an empty character set"
        )));
    }

    let mut generator = Generator;
    let password: String = (0..length)
        .map(|_| chars[generator.gen_range(0, chars.len())])
        .collect();
    Ok(password.into_bytes())
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
