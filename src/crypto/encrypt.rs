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
use util::data::SensitiveData;

pub fn encrypt<K: Key>(plaintext: SensitiveData, key: &K) -> Result<(secretbox::Nonce, Vec<u8>)> {
    let nonce = secretbox::gen_nonce();
    let ciphertext = secretbox::seal(&plaintext[..], &nonce, &key.get_key());
    Ok((nonce, ciphertext))
}
