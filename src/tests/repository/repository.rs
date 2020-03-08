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

use crate::error::*;
use crate::repository::*;
use crate::util::data::Secret;
use bdrck::testing::temp;
use sodiumoxide::randombytes::randombytes;
use std::ops::{Deref, DerefMut};

static TEST_REPO_DIR: &'static str = "pwm-test";

fn to_password(s: &str) -> Secret {
    s.as_bytes().to_vec()
}

struct TestRepository {
    _directory: temp::Dir,
    repository: Option<Repository>,
}

impl Deref for TestRepository {
    type Target = Repository;
    fn deref(&self) -> &Self::Target {
        self.repository.as_ref().unwrap()
    }
}

impl DerefMut for TestRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.repository.as_mut().unwrap()
    }
}

impl Drop for TestRepository {
    fn drop(&mut self) {
        // Forcefully drop the Repository *before* dropping (and thus deleting) the
        // temp directory.
        self.repository = None
    }
}

impl TestRepository {
    fn new(password: &str) -> Result<TestRepository> {
        let directory = temp::Dir::new(TEST_REPO_DIR)?;
        let repository = Repository::new(directory.path(), true, Some(to_password(password)))?;
        Ok(TestRepository {
            _directory: directory,
            repository: Some(repository),
        })
    }
}

#[test]
fn test_wrong_master_password_fails() {
    let repository_dir = temp::Dir::new(TEST_REPO_DIR).unwrap();
    let good = to_password("foobar");
    let bad = to_password("barbaz");
    let path = "test";

    {
        let mut repository = Repository::new(repository_dir.path(), true, Some(good)).unwrap();
        let path = repository.path(path).unwrap();
        repository
            .write_encrypt(&path, to_password("Hello, world!"))
            .unwrap();
    }

    let repository = Repository::new(repository_dir.path(), false, Some(bad)).unwrap();
    let path = repository.path(path).unwrap();
    let read_result = repository.read_decrypt(&path);
    assert!(read_result.is_err());
}

#[test]
fn test_write_read_round_trip() {
    let repository_dir = temp::Dir::new(TEST_REPO_DIR).unwrap();
    let pw = to_password("foobar");
    let path = "test";
    let plaintext = randombytes(1024);

    {
        let mut repository =
            Repository::new(repository_dir.path(), true, Some(pw.clone())).unwrap();
        let absolute_path = repository.path(path).unwrap();
        repository
            .write_encrypt(&absolute_path, plaintext.clone())
            .unwrap();
    }

    let repository = Repository::new(repository_dir.path(), false, Some(pw)).unwrap();
    let absolute_path = repository.path(path).unwrap();
    let output_plaintext = repository.read_decrypt(&absolute_path).unwrap();
    assert_eq!(&plaintext[..], &output_plaintext[..]);
}

#[test]
fn test_repository_listing() {
    let mut t = TestRepository::new("foobar").unwrap();
    let plaintext = randombytes(1024);

    let absolute_path = t.path("foo/1").unwrap();
    t.write_encrypt(&absolute_path, plaintext.clone()).unwrap();
    let absolute_path = t.path("bar/2").unwrap();
    t.write_encrypt(&absolute_path, plaintext.clone()).unwrap();
    let absolute_path = t.path("3").unwrap();
    t.write_encrypt(&absolute_path, plaintext.clone()).unwrap();
    let absolute_path = t.path("foo/bar/4").unwrap();
    t.write_encrypt(&absolute_path, plaintext.clone()).unwrap();

    let listing: Vec<String> = t
        .list(None)
        .unwrap()
        .iter()
        .map(|p| p.to_str().unwrap().to_owned())
        .collect();

    assert_eq!(
        vec![
            "3".to_owned(),
            "bar/2".to_owned(),
            "foo/1".to_owned(),
            "foo/bar/4".to_owned(),
        ],
        listing
    );
}

#[test]
fn test_remove() {
    let mut t = TestRepository::new("foobar").unwrap();
    let absolute_path = t.path("test").unwrap();
    t.write_encrypt(&absolute_path, randombytes(1024)).unwrap();

    let listing: Vec<String> = t
        .list(None)
        .unwrap()
        .iter()
        .map(|p| p.to_str().unwrap().to_owned())
        .collect();
    assert_eq!(vec!["test".to_owned()], listing);

    let path = t.path("test").unwrap();
    t.remove(&path).unwrap();
    let listing: Vec<String> = t
        .list(None)
        .unwrap()
        .iter()
        .map(|p| p.to_str().unwrap().to_owned())
        .collect();
    assert!(listing.is_empty());
}

#[test]
fn test_adding_duplicate_key() {
    let mut t = TestRepository::new("foobar").unwrap();
    assert!(t.add_password_key(Some(to_password("foobar"))).is_err());
}

#[test]
fn test_adding_key_succeeds() {
    let repository_dir = temp::Dir::new(TEST_REPO_DIR).unwrap();
    let pwa = to_password("foobar");
    let pwb = to_password("barbaz");
    let path = "test";
    let plaintext = randombytes(1024);

    {
        let mut repository = Repository::new(repository_dir.path(), true, Some(pwa)).unwrap();
        let path = repository.path(path).unwrap();
        repository.write_encrypt(&path, plaintext.clone()).unwrap();

        repository.add_password_key(Some(pwb.clone())).unwrap();
    }

    let repository = Repository::new(repository_dir.path(), false, Some(pwb)).unwrap();
    let path = repository.path(path).unwrap();
    let output_plaintext = repository.read_decrypt(&path).unwrap();
    assert_eq!(&plaintext[..], &output_plaintext[..]);
}

#[test]
fn test_removing_only_key() {
    let mut t = TestRepository::new("foobar").unwrap();
    assert!(t.remove_password_key(Some(to_password("foobar"))).is_err());
}

#[test]
fn test_removing_unused_key() {
    let mut t = TestRepository::new("foobar").unwrap();
    assert!(t.remove_password_key(Some(to_password("barbaz"))).is_err());
}

#[test]
fn test_removing_key_succeeds() {
    let repository_dir = temp::Dir::new(TEST_REPO_DIR).unwrap();
    let pwa = to_password("foobar");
    let pwb = to_password("barbaz");
    let path = "test";
    let plaintext = randombytes(1024);

    {
        let mut repository =
            Repository::new(repository_dir.path(), true, Some(pwa.clone())).unwrap();
        let path = repository.path(path).unwrap();
        repository.write_encrypt(&path, plaintext.clone()).unwrap();

        repository.add_password_key(Some(pwb.clone())).unwrap();
        repository.remove_password_key(Some(pwa.clone())).unwrap();
    }

    {
        // Accessing the repository with the old key should fail.
        let repository = Repository::new(repository_dir.path(), false, Some(pwa.clone())).unwrap();
        let path = repository.path(path).unwrap();
        assert!(repository.read_decrypt(&path).is_err());
    }

    // Accessing the repository with the new key should still succeed.
    let repository = Repository::new(repository_dir.path(), false, Some(pwb)).unwrap();
    let path = repository.path(path).unwrap();
    let output_plaintext = repository.read_decrypt(&path).unwrap();
    assert_eq!(&plaintext[..], &output_plaintext[..]);
}
