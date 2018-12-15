// Copyright 2015 Axel Rasmussen
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bdrck::testing::temp;
use configuration::*;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    // The unit tests in this file MUST be run one-at-a-time, since they all access our
    // global singleton configuration state.
    static ref CONFIGURATION_TESTS_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn test_get_and_set() {
    let _guard = match CONFIGURATION_TESTS_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let file = temp::File::new_file().unwrap();
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
    assert_eq!(PathBuf::from("/home/foo/bar"), config.default_repository.unwrap());
}

#[test]
fn test_get_value_as_str() {
    let _guard = match CONFIGURATION_TESTS_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let file = temp::File::new_file().unwrap();
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

    assert_eq!(
        "/home/foo/bar",
        get_value_as_str(DEFAULT_REPOSITORY_KEY).unwrap()
    );

    // While we're at it, also test that getting an invalid key is an error.
    assert!(get_value_as_str("bogus key").is_err());
}

#[test]
fn test_reset() {
    let _guard = match CONFIGURATION_TESTS_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let file = temp::File::new_file().unwrap();
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
        assert_eq!(PathBuf::from("/home/foo/bar"), config.default_repository.unwrap());

        reset().unwrap();
    }

    let _handle = SingletonHandle::new(Some(path.as_path())).unwrap();

    let config = get().unwrap();
    assert!(config.default_repository.is_none());
}
