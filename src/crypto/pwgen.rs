use crypto::rng::Generator;
use error::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use util::data::SensitiveData;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CharacterSet {
    Letters,
    Numbers,
    Symbols,
}

lazy_static! {
    static ref CHARACTER_SET: HashMap<CharacterSet, Vec<char>> = {
        let mut m = HashMap::new();
        m.insert(CharacterSet::Letters,
                 "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect());
        m.insert(CharacterSet::Numbers, "0123456789".chars().collect());
        m.insert(CharacterSet::Symbols, "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect());
        m
    };
}

pub const RECOMMENDED_MINIMUM_PASSWORD_LENGTH: usize = 16;

pub fn generate_password(length: usize,
                         charsets: &[CharacterSet],
                         exclude: &[char])
                         -> Result<SensitiveData> {
    if length == 0 {
        bail!("Refusing to generate a password of length 0");
    }

    let exclude: HashSet<char> = exclude.iter().cloned().collect();
    let chars: Vec<char> = charsets.iter()
        .flat_map(|cs| CHARACTER_SET.get(cs).unwrap().iter())
        .filter(|c| !exclude.contains(c))
        .cloned()
        .collect();
    if chars.is_empty() {
        bail!("Cannot generate passwords from an empty character set");
    }

    let mut generator = Generator;
    let password: String =
        (0..length).map(|_| chars[generator.gen_range(0, chars.len())]).collect();
    Ok(SensitiveData::from(password.into_bytes()))
}
