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
use sodiumoxide::crypto::pwhash;
use sodiumoxide::crypto::pwhash::{MemLimit, OpsLimit, Salt};
use sodiumoxide::crypto::secretbox;
use util::data::SensitiveData;

pub trait Key {
    fn get_key(&self) -> &secretbox::Key;
}

pub struct PasswordKey {
    salt: Salt,
    key: secretbox::Key,
}

impl PasswordKey {
    pub fn new(password: SensitiveData,
               salt: Option<Salt>,
               ops: Option<OpsLimit>,
               mem: Option<MemLimit>)
               -> Result<PasswordKey> {
        let salt: Salt = salt.unwrap_or(pwhash::gen_salt());
        let mut key = secretbox::Key([0; secretbox::KEYBYTES]);
        {
            let secretbox::Key(ref mut kb) = key;
            let result = pwhash::derive_key(kb,
                                            &password[..],
                                            &salt,
                                            ops.unwrap_or(pwhash::OPSLIMIT_INTERACTIVE),
                                            mem.unwrap_or(pwhash::MEMLIMIT_INTERACTIVE));
            if result.is_err() {
                // NOTE: We handle this error gracefully, but in reality (by inspecting the
                // libsodium source code) the only way this can actually fail is if the input
                // password is *enormous*. So, this won't really fail in practice.
                bail!("Deriving key from password failed");
            }
        }

        Ok(PasswordKey {
            salt: salt,
            key: key,
        })
    }

    pub fn get_salt(&self) -> &Salt { &self.salt }
}

impl Key for PasswordKey {
    fn get_key(&self) -> &secretbox::Key { &self.key }
}
