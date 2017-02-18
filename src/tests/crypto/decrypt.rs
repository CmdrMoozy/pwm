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

use ::crypto::decrypt::*;
use ::crypto::encrypt::*;
use crypto::key::Key;
use sodiumoxide::randombytes::randombytes;
use util::data::SensitiveData;

#[test]
fn test_decrypting_with_wrong_key_fails() {
    let key = Key::password_key(SensitiveData::from("foobar".as_bytes().to_vec()),
                                None,
                                None,
                                None)
        .unwrap();
    let plaintext = SensitiveData::from(randombytes(1024));
    let (nonce, ciphertext) = encrypt(plaintext, &key).ok().unwrap();

    let wrong_key = Key::password_key(SensitiveData::from("raboof".as_bytes().to_vec()),
                                      None,
                                      None,
                                      None)
        .unwrap();
    let decrypted_result = decrypt(ciphertext.as_slice(), &nonce, &wrong_key);
    assert!(decrypted_result.is_err());
}
