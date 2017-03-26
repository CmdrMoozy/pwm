use repository::Repository;
use ::repository::serde::*;
use sodiumoxide::randombytes::randombytes;
use tests::tempdir::TempDir;
use util::data::SensitiveData;

#[test]
fn test_export_import_round_trip_ascii() {
    let serialized: String;
    let plaintext = "arbitrary test password".to_owned();
    let plaintext_sd = SensitiveData::from(plaintext.as_bytes().to_vec());

    let paths: Vec<&'static str> = vec!["foo/1", "bar/2", "3", "foo/bar/4"];

    {
        let repository_dir = TempDir::new("pwm-test").unwrap();
        let repository = Repository::new(repository_dir.path(),
                                         true,
                                         Some(SensitiveData::from("foobar".as_bytes().to_vec())))
            .unwrap();
        for path in &paths {
            repository.write_encrypt(&repository.path(path).unwrap(), plaintext_sd.clone())
                .unwrap();
        }
        serialized = export_serialize(&repository).unwrap();
    }

    let repository_dir = TempDir::new("pwm-test").unwrap();
    let repository = Repository::new(repository_dir.path(),
                                     true,
                                     Some(SensitiveData::from("raboof".as_bytes().to_vec())))
        .unwrap();
    assert_eq!(0, repository.list(None).unwrap().len());
    import_deserialize(&repository, serialized.as_str()).unwrap();
    for path in &paths {
        assert_eq!(plaintext_sd,
                   repository.read_decrypt(&repository.path(path).unwrap()).unwrap());
    }
}

#[test]
fn test_export_import_round_trip_binary() {
    let serialized: String;
    let plaintext = SensitiveData::from(randombytes(1024));

    let paths: Vec<&'static str> = vec!["foo/1", "bar/2", "3", "foo/bar/4"];

    {
        let repository_dir = TempDir::new("pwm-test").unwrap();
        let repository = Repository::new(repository_dir.path(),
                                         true,
                                         Some(SensitiveData::from("foobar".as_bytes().to_vec())))
            .unwrap();
        for path in &paths {
            repository.write_encrypt(&repository.path(path).unwrap(), plaintext.clone()).unwrap();
        }
        serialized = export_serialize(&repository).unwrap();
    }

    let repository_dir = TempDir::new("pwm-test").unwrap();
    let repository = Repository::new(repository_dir.path(),
                                     true,
                                     Some(SensitiveData::from("raboof".as_bytes().to_vec())))
        .unwrap();
    assert_eq!(0, repository.list(None).unwrap().len());
    import_deserialize(&repository, serialized.as_str()).unwrap();
    for path in &paths {
        assert_eq!(plaintext,
                   repository.read_decrypt(&repository.path(path).unwrap()).unwrap());
    }
}
