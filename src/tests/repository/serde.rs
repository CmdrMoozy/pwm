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

use crate::repository::serde::*;
use crate::repository::Repository;
use crate::tests::{random_secret, str_secret};
use bdrck::testing::temp;

#[test]
fn test_export_import_round_trip_ascii() {
    crate::init().unwrap();

    let serialized: String;
    let plaintext = str_secret("arbitrary test password");

    let paths: Vec<&'static str> = vec!["foo/1", "bar/2", "3", "foo/bar/4"];

    {
        let repository_dir = temp::Dir::new("pwm-test").unwrap();
        let mut repository =
            Repository::new(repository_dir.path(), true, Some(str_secret("foobar"))).unwrap();
        for path in &paths {
            let absolute_path = repository.path(path).unwrap();
            repository
                .write_encrypt(&absolute_path, plaintext.try_clone().unwrap(), None)
                .unwrap();
        }
        serialized = export_serialize(&mut repository).unwrap();
    }

    let repository_dir = temp::Dir::new("pwm-test").unwrap();
    let mut repository =
        Repository::new(repository_dir.path(), true, Some(str_secret("raboof"))).unwrap();
    assert_eq!(0, repository.list(None).unwrap().len());
    import_deserialize(&mut repository, serialized.as_str()).unwrap();
    for path in &paths {
        let absolute_path = repository.path(path).unwrap();
        unsafe {
            assert_eq!(
                plaintext.as_slice(),
                repository.read_decrypt(&absolute_path).unwrap().as_slice()
            );
        }
    }
}

#[test]
fn test_export_import_round_trip_binary() {
    crate::init().unwrap();

    let serialized: String;
    let plaintext = random_secret(1024);

    let paths: Vec<&'static str> = vec!["foo/1", "bar/2", "3", "foo/bar/4"];

    {
        let repository_dir = temp::Dir::new("pwm-test").unwrap();
        let mut repository =
            Repository::new(repository_dir.path(), true, Some(str_secret("foobar"))).unwrap();
        for path in &paths {
            let absolute_path = repository.path(path).unwrap();
            repository
                .write_encrypt(&absolute_path, plaintext.try_clone().unwrap(), None)
                .unwrap();
        }
        serialized = export_serialize(&mut repository).unwrap();
    }

    let repository_dir = temp::Dir::new("pwm-test").unwrap();
    let mut repository =
        Repository::new(repository_dir.path(), true, Some(str_secret("raboof"))).unwrap();
    assert_eq!(0, repository.list(None).unwrap().len());
    import_deserialize(&mut repository, serialized.as_str()).unwrap();
    for path in &paths {
        let absolute_path = repository.path(path).unwrap();
        unsafe {
            assert_eq!(
                plaintext.as_slice(),
                repository.read_decrypt(&absolute_path).unwrap().as_slice()
            );
        }
    }
}
