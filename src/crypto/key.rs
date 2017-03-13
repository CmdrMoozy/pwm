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

use error::Result;
use sodiumoxide::crypto::hash::{Digest, hash};
use sodiumoxide::crypto::pwhash::{self, MemLimit, OpsLimit, Salt};
use sodiumoxide::crypto::secretbox;
use sodiumoxide::randombytes::randombytes;
use util::data::SensitiveData;
use util::serde::{deserialize_binary, serialize_binary};

pub trait Key {
    /// Returns this Key's signature (i.e., a hash of the actual key material).
    /// This is just a direct hash in the case of a normal key, or the hash of
    /// the outer-most wrapping key in the case of a wrapped key. Note that,
    /// because of the latter case, this is not suitable for checking key
    /// equiality in all cases.
    fn get_signature(&self) -> &Digest;
}

#[derive(Clone, Deserialize, Serialize)]
pub struct NormalKey {
    key: secretbox::Key,
    signature: Digest,
}

impl NormalKey {
    /// This is a utility used to implement our various public constructors.
    /// This constructor builds a new NormalKey from the given raw bytes.
    fn from_bytes(data: Vec<u8>) -> Result<NormalKey> {
        let signature = hash(data.as_slice());
        let key = secretbox::Key::from_slice(data.as_slice());
        if key.is_none() {
            bail!("Building key from raw data failed");
        }

        Ok(NormalKey {
            key: key.unwrap(),
            signature: signature,
        })
    }

    pub fn new_random() -> Result<NormalKey> { Self::from_bytes(randombytes(secretbox::KEYBYTES)) }

    pub fn new_password(password: SensitiveData,
                        salt: Option<Salt>,
                        ops: Option<OpsLimit>,
                        mem: Option<MemLimit>)
                        -> Result<NormalKey> {
        let salt = salt.unwrap_or_else(pwhash::gen_salt);
        let ops = ops.unwrap_or(pwhash::OPSLIMIT_INTERACTIVE);
        let mem = mem.unwrap_or(pwhash::MEMLIMIT_INTERACTIVE);

        let mut key_buffer = vec![0; secretbox::KEYBYTES];
        {
            let result =
                pwhash::derive_key(key_buffer.as_mut_slice(), &password[..], &salt, ops, mem);
            if result.is_err() {
                // NOTE: We handle this error gracefully, but in reality (by inspecting the
                // libsodium source code) the only way this can actually fail is if the input
                // password is *enormous*. So, this won't really fail in practice.
                bail!("Deriving key from password failed");
            }
        }

        Self::from_bytes(key_buffer)
    }

    pub fn encrypt(&self, plaintext: SensitiveData) -> (secretbox::Nonce, Vec<u8>) {
        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(&plaintext[..], &nonce, &self.key);
        (nonce, ciphertext)
    }

    pub fn decrypt(&self, ciphertext: &[u8], nonce: &secretbox::Nonce) -> Result<SensitiveData> {
        let result = secretbox::open(ciphertext, nonce, &self.key);
        if result.is_err() {
            bail!("Decryption with the provided key failed");
        }
        Ok(SensitiveData::from(result.ok().unwrap()))
    }

    pub fn wrap(self, key: &NormalKey) -> Result<WrappedKey> {
        let serialized = try!(serialize_binary(&self));
        let (nonce, encrypted) = key.encrypt(SensitiveData::from(serialized));
        Ok(WrappedKey {
            data: encrypted,
            nonce: nonce,
            signature: key.signature.clone(),
        })
    }
}

impl Key for NormalKey {
    fn get_signature(&self) -> &Digest { &self.signature }
}

#[derive(Deserialize, Serialize)]
pub struct WrappedKey {
    /// The raw wrapped bytes. This key needs to be unwrapped before these
    /// bytes can be used.
    data: Vec<u8>,
    /// The nonce used to encrypt this wrapped key.
    nonce: secretbox::Nonce,
    /// The signature of the key used to wrap this key.
    signature: Digest,
}

impl WrappedKey {
    pub fn unwrap(&self, key: &NormalKey) -> Result<NormalKey> {
        let decrypted = try!(key.decrypt(self.data.as_slice(), &self.nonce));
        deserialize_binary(&decrypted[..])
    }
}

impl Key for WrappedKey {
    fn get_signature(&self) -> &Digest { &self.signature }
}
