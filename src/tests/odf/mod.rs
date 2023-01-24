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

use crate::repository::path::Path as RepositoryPath;
use crate::repository::Repository;
use crate::tests::str_secret;
use bdrck::crypto::key::Nonce;
use bdrck::crypto::secret::Secret;
use flate2::read::GzDecoder;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Cursor;
use tar;
use tempfile::TempDir;

const TEST_REPO: &'static [u8] = include_bytes!("test-repository.tar.gz");
const TEST_REPO_SUBDIR: &'static str = "pwm-test";
static TEST_REPO_MASTER_PASSWORD: Lazy<Secret> = Lazy::new(|| str_secret("qwerty"));
const TEST_REPO_NONCE: &'static [u8] = &[
    231, 97, 13, 54, 159, 192, 85, 254, 94, 94, 227, 45, 31, 160, 149, 134, 241, 181, 52, 242, 241,
    87, 235, 245,
];
const TEST_REPO_PATH: &'static str = "foo/bar";
const TEST_REPO_NEW_PATH: &'static str = "bar/baz";
static TEST_REPO_PASSWORD: Lazy<Secret> = Lazy::new(|| str_secret("this is a test password"));

fn open_test_repo() -> (TempDir, Repository) {
    let tmp = tempfile::Builder::new()
        .prefix(env!("CARGO_PKG_NAME"))
        .tempdir()
        .expect("creating tempdir failed");

    {
        let cur = Cursor::new(TEST_REPO);
        let gz = GzDecoder::new(cur);
        let mut tar = tar::Archive::new(gz);
        tar.unpack(tmp.path())
            .expect("unpacking test repository failed");
    }

    let repo = Repository::new(
        tmp.path().join(TEST_REPO_SUBDIR),
        /*create=*/ false,
        Some(TEST_REPO_MASTER_PASSWORD.try_clone().unwrap()),
    )
    .expect("opening repository failed");

    (tmp, repo)
}

fn read_repo_file_raw(path: &RepositoryPath) -> (Option<Nonce>, Vec<u8>) {
    let mut f = File::open(path.absolute_path()).expect("opening file failed");
    rmp_serde::decode::from_read(&mut f).expect("decoding repository file failed")
}

// Verify we can read a password out of a previously created repository. The idea is to detect code
// changes which make us unable to interpret existing repositories.
#[test]
fn test_read_repository() {
    crate::init().unwrap();

    let (_tmp, repo) = open_test_repo();

    let path = repo
        .path(TEST_REPO_PATH)
        .expect("constructing repository path failed");

    {
        let mut list = repo.list(None).expect("listing repository contents failed");
        assert_eq!(1, list.len());
        assert_eq!(path.relative_path(), list.pop().unwrap().relative_path());
    }

    let stored = repo
        .read_decrypt(&path)
        .expect("retrieving stored password failed");
    let stored = String::from_utf8(unsafe { stored.as_slice().to_vec() })
        .expect("stored password is not valid utf-8");
    unsafe {
        assert_eq!(TEST_REPO_PASSWORD.as_slice(), stored.as_bytes());
    }
}

// Verify when we encrypt, the ciphertext we emit matches what we expect. The idea is to detect
// code changes which would cause us to change the ciphertext we emit when we write to a
// repository.
#[test]
fn test_write_repository() {
    crate::init().unwrap();

    let (_tmp, mut repo) = open_test_repo();

    let path = repo
        .path(TEST_REPO_NEW_PATH)
        .expect("constructing repository path failed");
    let reference_path = repo
        .path(TEST_REPO_PATH)
        .expect("constructing repository reference path failed");

    repo.write_encrypt(
        &path,
        TEST_REPO_PASSWORD.try_clone().unwrap(),
        Some(Nonce::from_slice(TEST_REPO_NONCE).expect("constructing nonce failed")),
    )
    .expect("storing new password failed");

    let (actual_nonce, actual_ciphertext) = read_repo_file_raw(&path);
    let (expected_nonce, expected_ciphertext) = read_repo_file_raw(&reference_path);

    assert_eq!(expected_nonce, actual_nonce);
    assert_eq!(expected_ciphertext.len(), actual_ciphertext.len());
    assert_eq!(expected_ciphertext, actual_ciphertext);
}
