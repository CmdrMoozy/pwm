use repository::*;
use sodiumoxide::randombytes::randombytes;
use std::path::PathBuf;
use tests::tempdir::TempDir;
use util::data::SensitiveData;

#[test]
fn test_wrong_master_password_fails() {
    let repository_dir = TempDir::new("pwm-test").unwrap();

    let good = SensitiveData::from("foobar".as_bytes().to_vec());
    let bad = SensitiveData::from("barbaz".as_bytes().to_vec());
    let path = PathBuf::from("test");

    {
        let repository = Repository::new(repository_dir.path(), true, Some(good)).unwrap();
        let path = repository.path(path.as_path()).unwrap();
        repository.write_encrypt(&path,
                           SensitiveData::from("Hello, world!".as_bytes().to_vec()))
            .unwrap();
    }

    let repository = Repository::new(repository_dir.path(), false, Some(bad)).unwrap();
    let path = repository.path(path.as_path()).unwrap();
    let read_result = repository.read_decrypt(&path);
    assert!(read_result.is_err());
}

#[test]
fn test_write_read_round_trip() {
    let repository_dir = TempDir::new("pwm-test").unwrap();
    let plaintext = SensitiveData::from(randombytes(1024));

    {
        let repository = Repository::new(repository_dir.path(),
                                         true,
                                         Some(SensitiveData::from("foobar".as_bytes().to_vec())))
            .unwrap();
        repository.write_encrypt(&repository.path("test").unwrap(), plaintext.clone()).unwrap();
    }

    {
        let repository = Repository::new(repository_dir.path(),
                                         false,
                                         Some(SensitiveData::from("foobar".as_bytes().to_vec())))
            .unwrap();
        let output_plaintext = repository.read_decrypt(&repository.path("test").unwrap()).unwrap();
        assert_eq!(&plaintext[..], &output_plaintext[..]);
    }
}

#[test]
fn test_repository_listing() {
    let repository_dir = TempDir::new("pwm-test").unwrap();
    let repository = Repository::new(repository_dir.path(),
                                     true,
                                     Some(SensitiveData::from("foobar".as_bytes().to_vec())))
        .unwrap();
    let plaintext = SensitiveData::from(randombytes(1024));

    repository.write_encrypt(&repository.path("foo/1").unwrap(), plaintext.clone()).unwrap();
    repository.write_encrypt(&repository.path("bar/2").unwrap(), plaintext.clone()).unwrap();
    repository.write_encrypt(&repository.path("3").unwrap(), plaintext.clone()).unwrap();
    repository.write_encrypt(&repository.path("foo/bar/4").unwrap(), plaintext.clone()).unwrap();

    let listing: Vec<String> =
        repository.list(None).unwrap().iter().map(|p| p.to_str().unwrap().to_owned()).collect();

    assert_eq!(vec!["3".to_owned(),
                    "bar/2".to_owned(),
                    "foo/1".to_owned(),
                    "foo/bar/4".to_owned()],
               listing);
}

#[test]
fn test_remove() {
    let repository_dir = TempDir::new("pwm-test").unwrap();

    let repository = Repository::new(repository_dir.path(),
                                     true,
                                     Some(SensitiveData::from("foobar".as_bytes().to_vec())))
        .unwrap();
    repository.write_encrypt(&repository.path("test").unwrap(),
                       SensitiveData::from(randombytes(1024)))
        .unwrap();

    let listing: Vec<String> =
        repository.list(None).unwrap().iter().map(|p| p.to_str().unwrap().to_owned()).collect();
    assert_eq!(vec!["test".to_owned()], listing);

    repository.remove(&repository.path("test").unwrap()).unwrap();
    let listing: Vec<String> =
        repository.list(None).unwrap().iter().map(|p| p.to_str().unwrap().to_owned()).collect();
    assert!(listing.is_empty());
}
