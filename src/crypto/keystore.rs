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
use sodiumoxide::crypto::secretbox;
use std::fs::File;
use std::path::Path;
use util::data::SensitiveData;
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
    pub token_nonce: secretbox::Nonce,
    pub token: Vec<u8>,
    pub wrapped_keys: Vec<Key>,
}

impl EncryptedContents {
    pub fn new(master_key: &Key) -> Result<EncryptedContents> {
        let (nonce, encrypted) =
            try!(master_key.encrypt(SensitiveData::from(AUTH_TOKEN_CONTENTS.clone())));
        Ok(EncryptedContents {
            token_nonce: nonce,
            token: encrypted,
            wrapped_keys: Vec::new(),
        })
    }

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

    pub fn is_master_key(&self, key: &Key) -> Result<bool> {
        let decrypted = try!(key.decrypt(self.token.as_slice(), &self.token_nonce));
        Ok(&decrypted[..] == AUTH_TOKEN_CONTENTS.as_slice())
    }

    pub fn add(&mut self, wrapped_key: Key) { self.wrapped_keys.push(wrapped_key) }
}

pub struct KeyStore {
    master_key: Key,
    encrypted_contents: EncryptedContents,
}

impl KeyStore {
    pub fn new() -> Result<KeyStore> {
        let master_key = try!(Key::random_key());
        let encrypted_contents = try!(EncryptedContents::new(&master_key));

        Ok(KeyStore {
            master_key: master_key,
            encrypted_contents: encrypted_contents,
        })
    }

    pub fn open<P: AsRef<Path>>(path: P, wrap_key: &Key) -> Result<KeyStore> {
        let contents = try!(EncryptedContents::open(path));
        let mut master_key: Option<Key> = None;
        for wrapped_key in contents.wrapped_keys.iter() {
            let key = try!(wrapped_key.unwrap(wrap_key));
            if try!(contents.is_master_key(&key)) {
                master_key = Some(key);
            }
        }

        if master_key.is_some() {
            return Ok(KeyStore {
                master_key: master_key.unwrap(),
                encrypted_contents: contents,
            });
        }
        bail!("Failed to unwrap master key with the provided wrapping key.");
    }

    pub fn get_key(&self) -> &Key { &self.master_key }
}
