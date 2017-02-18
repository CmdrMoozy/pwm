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

use bincode::{deserialize, serialize};
use bincode::SizeLimit;
use error::Result;
use sodiumoxide::crypto::pwhash;
use sodiumoxide::crypto::pwhash::{MemLimit, OpsLimit, Salt};
use sodiumoxide::crypto::secretbox;
use sodiumoxide::randombytes::randombytes;
use util::data::SensitiveData;

#[derive(Deserialize, Serialize)]
pub struct Key {
    wrap_nonce: Option<secretbox::Nonce>,
    data: Vec<u8>,
    key: Option<secretbox::Key>,
}

impl Key {
    fn new(wrap_nonce: Option<secretbox::Nonce>, data: Vec<u8>) -> Key {
        let key = if wrap_nonce.is_some() {
            None
        } else {
            secretbox::Key::from_slice(&data[..])
        };

        Key {
            wrap_nonce: wrap_nonce,
            data: data,
            key: key,
        }
    }

    pub fn random_key() -> Key { Key::new(None, randombytes(secretbox::KEYBYTES)) }

    pub fn password_key(password: SensitiveData,
                        salt: Option<Salt>,
                        ops: Option<OpsLimit>,
                        mem: Option<MemLimit>)
                        -> Result<Key> {
        let salt = salt.unwrap_or_else(pwhash::gen_salt);
        let ops = ops.unwrap_or(pwhash::OPSLIMIT_INTERACTIVE);
        let mem = mem.unwrap_or(pwhash::MEMLIMIT_INTERACTIVE);

        let mut key_buffer = vec![0; secretbox::KEYBYTES];
        {
            let result =
                pwhash::derive_key(key_buffer.as_mut_slice(), &password[..], &salt, ops, mem);
            if result.is_err() {
                // NOTE: We handle this error gracefully, but in reality (by inspecting the
                // libsodium
                // source code) the only way this can actually fail is if the input password is
                // *enormous*. So, this won't really fail in practice.
                bail!("Deriving key from password failed");
            }
        }

        Ok(Key::new(None, key_buffer))
    }

    pub fn get_key(&self) -> Result<&secretbox::Key> {
        match self.key.as_ref() {
            Some(k) => Ok(k),
            None => bail!("Cannot build encryption key from wrapped key."),
        }
    }

    pub fn encrypt(&self, plaintext: SensitiveData) -> Result<(secretbox::Nonce, Vec<u8>)> {
        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(&plaintext[..], &nonce, try!(self.get_key()));
        Ok((nonce, ciphertext))
    }

    pub fn decrypt(&self, ciphertext: &[u8], nonce: &secretbox::Nonce) -> Result<SensitiveData> {
        let result = secretbox::open(ciphertext, nonce, try!(self.get_key()));
        if result.is_err() {
            bail!("Ciphertext failed key verification");
        }
        Ok(SensitiveData::from(result.ok().unwrap()))
    }

    pub fn wrap(self, wrap_key: &Key) -> Result<Key> {
        let serialized = match serialize(&self, SizeLimit::Infinite) {
            Err(_) => bail!("Serializing key failed"),
            Ok(s) => s,
        };
        let (nonce, encrypted) = try!(wrap_key.encrypt(SensitiveData::from(serialized)));
        Ok(Key::new(Some(nonce), encrypted))
    }

    pub fn unwrap(self, wrap_key: &Key) -> Result<Key> {
        if self.wrap_nonce.is_none() {
            bail!("Cannot unwrap key without nonce");
        }

        let decrypted =
            try!(wrap_key.decrypt(self.data.as_slice(), self.wrap_nonce.as_ref().unwrap()));
        Ok(match deserialize(&decrypted[..]) {
            Err(_) => bail!("Deserializing key failed"),
            Ok(d) => d,
        })
    }
}
