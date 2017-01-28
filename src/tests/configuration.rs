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

use ::configuration::*;
use std::fs;
use std::path::PathBuf;
use ::tests::tempfile;

#[test]
fn test_get_and_set() {
    let file = tempfile::NamedTempFile::new().ok().unwrap();
    let path: PathBuf = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized configuration struct.
    fs::remove_file(path.as_path()).unwrap();

    {
        let _handle = SingletonHandle::new(Some(path.as_path())).unwrap();

        let config = get().unwrap();
        assert!(config.default_repository.is_none());

        set(DEFAULT_REPOSITORY_KEY, "/home/foo/bar").unwrap();

        // While we're at it, also test that setting an invalid key is an error.
        assert!(set("bogus key", "value").is_err());
    }

    let _handle = SingletonHandle::new(Some(path.as_path())).unwrap();

    let config = get().unwrap();
    assert_eq!("/home/foo/bar", config.default_repository.unwrap());
}

#[test]
fn test_get_value_as_str() {
    let file = tempfile::NamedTempFile::new().ok().unwrap();
    let path: PathBuf = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized configuration struct.
    fs::remove_file(path.as_path()).unwrap();

    {
        let _handle = SingletonHandle::new(Some(path.as_path())).unwrap();

        let config = get().unwrap();
        assert!(config.default_repository.is_none());

        set(DEFAULT_REPOSITORY_KEY, "/home/foo/bar").unwrap();
    }

    let _handle = SingletonHandle::new(Some(path.as_path())).unwrap();

    assert_eq!("/home/foo/bar",
               get_value_as_str(DEFAULT_REPOSITORY_KEY).unwrap());

    // While we're at it, also test that getting an invalid key is an error.
    assert!(get_value_as_str("bogus key").is_err());
}

#[test]
fn test_reset() {
    let file = tempfile::NamedTempFile::new().ok().unwrap();
    let path: PathBuf = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized configuration struct.
    fs::remove_file(path.as_path()).unwrap();

    {
        let _handle = SingletonHandle::new(Some(path.as_path())).unwrap();

        let config = get().unwrap();
        assert!(config.default_repository.is_none());

        set(DEFAULT_REPOSITORY_KEY, "/home/foo/bar").unwrap();

        // While we're at it, also test that setting an invalid key is an error.
        assert!(set("bogus key", "value").is_err());
    }

    {
        let _handle = SingletonHandle::new(Some(path.as_path())).unwrap();

        let config = get().unwrap();
        assert_eq!("/home/foo/bar", config.default_repository.unwrap());

        reset().unwrap();
    }

    let _handle = SingletonHandle::new(Some(path.as_path())).unwrap();

    let config = get().unwrap();
    assert!(config.default_repository.is_none());
}
