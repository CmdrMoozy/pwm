use error::*;
use repository::*;
use sodiumoxide::randombytes::randombytes;
use std::ops::{Deref, DerefMut};
use tests::tempdir::TempDir;
use util::data::SensitiveData;

static TEST_REPO_DIR: &'static str = "pwm-test";

fn to_password(s: &str) -> SensitiveData { SensitiveData::from(s.as_bytes().to_vec()) }

struct TestRepository {
    _directory: TempDir,
    repository: Option<Repository>,
}

impl Deref for TestRepository {
    type Target = Repository;
    fn deref(&self) -> &Self::Target { self.repository.as_ref().unwrap() }
}

impl DerefMut for TestRepository {
    fn deref_mut(&mut self) -> &mut Self::Target { self.repository.as_mut().unwrap() }
}

impl Drop for TestRepository {
    fn drop(&mut self) {
        // Forcefully drop the Repository *before* dropping (and thus deleting) the
        // temp directory.
        self.repository = None
    }
}

impl TestRepository {
    pub fn new(password: &str) -> Result<TestRepository> {
        let directory = try!(TempDir::new(TEST_REPO_DIR));
        let repository = try!(Repository::new(directory.path(), true, Some(to_password(password))));
        Ok(TestRepository {
            _directory: directory,
            repository: Some(repository),
        })
    }
}

#[test]
fn test_wrong_master_password_fails() {
    let repository_dir = TempDir::new(TEST_REPO_DIR).unwrap();
    let good = to_password("foobar");
    let bad = to_password("barbaz");
    let path = "test";

    {
        let repository = Repository::new(repository_dir.path(), true, Some(good)).unwrap();
        let path = repository.path(path).unwrap();
        repository.write_encrypt(&path, to_password("Hello, world!")).unwrap();
    }

    let repository = Repository::new(repository_dir.path(), false, Some(bad)).unwrap();
    let path = repository.path(path).unwrap();
    let read_result = repository.read_decrypt(&path);
    assert!(read_result.is_err());
}

#[test]
fn test_write_read_round_trip() {
    let repository_dir = TempDir::new(TEST_REPO_DIR).unwrap();
    let pw = to_password("foobar");
    let path = "test";
    let plaintext = SensitiveData::from(randombytes(1024));

    {
        let repository = Repository::new(repository_dir.path(), true, Some(pw.clone())).unwrap();
        repository.write_encrypt(&repository.path(path).unwrap(), plaintext.clone()).unwrap();
    }

    let repository = Repository::new(repository_dir.path(), false, Some(pw)).unwrap();
    let output_plaintext = repository.read_decrypt(&repository.path(path).unwrap()).unwrap();
    assert_eq!(&plaintext[..], &output_plaintext[..]);
}

#[test]
fn test_repository_listing() {
    let t = TestRepository::new("foobar").unwrap();
    let plaintext = SensitiveData::from(randombytes(1024));

    t.write_encrypt(&t.path("foo/1").unwrap(), plaintext.clone()).unwrap();
    t.write_encrypt(&t.path("bar/2").unwrap(), plaintext.clone()).unwrap();
    t.write_encrypt(&t.path("3").unwrap(), plaintext.clone()).unwrap();
    t.write_encrypt(&t.path("foo/bar/4").unwrap(), plaintext.clone()).unwrap();

    let listing: Vec<String> =
        t.list(None).unwrap().iter().map(|p| p.to_str().unwrap().to_owned()).collect();

    assert_eq!(vec!["3".to_owned(),
                    "bar/2".to_owned(),
                    "foo/1".to_owned(),
                    "foo/bar/4".to_owned()],
               listing);
}

#[test]
fn test_remove() {
    let t = TestRepository::new("foobar").unwrap();
    t.write_encrypt(&t.path("test").unwrap(),
                       SensitiveData::from(randombytes(1024)))
        .unwrap();

    let listing: Vec<String> =
        t.list(None).unwrap().iter().map(|p| p.to_str().unwrap().to_owned()).collect();
    assert_eq!(vec!["test".to_owned()], listing);

    t.remove(&t.path("test").unwrap()).unwrap();
    let listing: Vec<String> =
        t.list(None).unwrap().iter().map(|p| p.to_str().unwrap().to_owned()).collect();
    assert!(listing.is_empty());
}
