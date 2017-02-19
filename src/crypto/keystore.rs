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

use crypto::key::Key;
use error::Result;
use std::fs::File;
use std::path::Path;
use util::serde::{deserialize_binary, serialize_binary};

lazy_static! {
    /// This token is used to verify that authentication was successful. We encrypt it with a master
    /// key which we then wrap with user key(s), so we can verify that the user presented a valid
    /// key by trying to decrypt this token.
    static ref AUTH_TOKEN_CONTENTS: Vec<u8> =
        "3c017f717b39247c351154a41d2850e4187284da4b928f13c723d54440ba2dfe".bytes().collect();
}

#[derive(Deserialize, Serialize)]
struct EncryptedContents {
    pub token: Vec<u8>,
    pub wrapped_keys: Vec<Vec<u8>>,
}

impl EncryptedContents {
    pub fn new() -> EncryptedContents { Self::default() }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<EncryptedContents> {
        use std::io::Read;
        let mut file = try!(File::open(path));
        let mut contents: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut contents));
        deserialize_binary(contents.as_slice())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        use std::io::Write;
        let data = try!(serialize_binary(self));
        let mut file = try!(File::create(path));
        Ok(try!(file.write_all(data.as_slice())))
    }
}

impl Default for EncryptedContents {
    fn default() -> EncryptedContents {
        EncryptedContents {
            token: AUTH_TOKEN_CONTENTS.clone(),
            wrapped_keys: Vec::new(),
        }
    }
}

pub struct KeyStore {
    master_key: Key,
}

impl KeyStore {
    pub fn new() -> KeyStore { Self::default() }

    pub fn get_key(&self) -> &Key { &self.master_key }
}

impl Default for KeyStore {
    fn default() -> KeyStore { KeyStore { master_key: Key::random_key() } }
}
