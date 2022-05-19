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

use crate::repository::Repository;
use flate2::read::GzDecoder;
use std::io::Cursor;
use tar;
use tempdir::TempDir;

const TEST_REPO: &'static [u8] = include_bytes!("test-repository.tar.gz");
const TEST_REPO_SUBDIR: &'static str = "pwm-test";
const TEST_REPO_MASTER_PASSWORD: &'static str = "qwerty";
const TEST_REPO_PATH: &'static str = "foo/bar";
const TEST_REPO_PASSWORD: &'static str = "this is a test password";

// Verify we can read a password out of a previously created repository. The idea is to detect code
// changes which make us unable to interpret existing repositories.
#[test]
fn test_read_repository() {
    let tmp = TempDir::new(env!("CARGO_PKG_NAME")).expect("creating tempdir failed");

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
        Some(TEST_REPO_MASTER_PASSWORD.to_string().into_bytes()),
    )
    .expect("opening repository failed");
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
    let stored = String::from_utf8(stored).expect("stored password is not valid utf-8");
    assert_eq!(TEST_REPO_PASSWORD, stored);
}
