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

use crate::crypto::configuration::Configuration;
use crate::repository::keystore::*;
use crate::tests::str_secret;
use bdrck::testing::temp;

static TEST_KEYSTORE_DIR: &'static str = "pwm-test";
static TEST_KEYSTORE_FILE: &'static str = "keystore";

#[test]
fn test_creation() {
    let directory = temp::Dir::new(TEST_KEYSTORE_DIR).unwrap();
    let keystore = get_keystore(
        directory.sub_path(TEST_KEYSTORE_FILE).unwrap(),
        /*allow_create=*/ true,
        &Configuration::default(),
        Some(str_secret("foo")),
    )
    .unwrap();
    assert!(keystore.is_open());
    assert_eq!(1, keystore.iter_wrapped_keys().count());
}

#[test]
fn test_opening_existing() {
    let directory = temp::Dir::new(TEST_KEYSTORE_DIR).unwrap();
    let config = Configuration::default();

    {
        let _keystore = get_keystore(
            directory.sub_path(TEST_KEYSTORE_FILE).unwrap(),
            /*allow_create=*/ true,
            &config,
            Some(str_secret("foo")),
        )
        .unwrap();
    }

    let keystore = get_keystore(
        directory.sub_path(TEST_KEYSTORE_FILE).unwrap(),
        /*allow_create=*/ false,
        &config,
        Some(str_secret("foo")),
    )
    .unwrap();
    assert!(keystore.is_open());
    assert_eq!(1, keystore.iter_wrapped_keys().count());
}

#[test]
fn test_open_bad_key_fails() {
    let directory = temp::Dir::new(TEST_KEYSTORE_DIR).unwrap();
    let config = Configuration::default();

    {
        let _keystore = get_keystore(
            directory.sub_path(TEST_KEYSTORE_FILE).unwrap(),
            /*allow_create=*/ true,
            &config,
            Some(str_secret("foo")),
        )
        .unwrap();
    }

    assert!(get_keystore(
        directory.sub_path(TEST_KEYSTORE_FILE).unwrap(),
        /*allow_create=*/ false,
        &config,
        Some(str_secret("bar"))
    )
    .is_err());
}
