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

use crypto::key::*;
use crypto::keystore::*;
use std::fs;
use tests::tempfile;

#[test]
fn test_keystore_save_round_trip() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized KeyStore.
    fs::remove_file(path.as_path()).unwrap();

    let wrap_key = Key::random_key().unwrap();
    let master_key: Option<Key>;

    {
        let keystore = KeyStore::open_or_new(path.as_path(), &wrap_key).unwrap();
        master_key = Some(keystore.get_key().clone());
    }

    {
        let mut keystore = KeyStore::open_or_new(path.as_path(), &wrap_key).unwrap();
        assert_eq!(master_key.as_ref().unwrap().get_key().unwrap(),
                   keystore.get_key().get_key().unwrap());
        assert!(keystore.remove(&wrap_key));
    }
}

#[test]
fn test_add_duplicate_key() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized KeyStore.
    fs::remove_file(path.as_path()).unwrap();

    let wrap_key = Key::random_key().unwrap();
    // Note that creating a new KeyStore automatically adds the given key.
    let mut keystore = KeyStore::open_or_new(path.as_path(), &wrap_key).unwrap();
    // Check that adding the same key again doesn't work.
    assert!(!keystore.add(&wrap_key).unwrap());
}

#[test]
fn test_remove_unused_key() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized KeyStore.
    fs::remove_file(path.as_path()).unwrap();

    let wrap_key = Key::random_key().unwrap();
    let mut keystore = KeyStore::open_or_new(path.as_path(), &wrap_key).unwrap();
    // Test that removing some other key returns false.
    let other_key = Key::random_key().unwrap();
    assert!(!keystore.remove(&other_key));
}
