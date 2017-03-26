use crypto::key::*;
use crypto::keystore::*;
use std::fs;
use tests::tempfile;

#[test]
fn test_keystore_save_round_trip() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized KeyStore.
    fs::remove_file(path.as_path()).unwrap();

    let wrap_key = NormalKey::new_random().unwrap();
    let master_key: Option<NormalKey>;

    {
        let keystore = KeyStore::open_or_new(path.as_path(), &wrap_key).unwrap();
        master_key = Some(keystore.get_key().clone());
    }

    {
        let keystore = KeyStore::open_or_new(path.as_path(), &wrap_key).unwrap();
        assert_eq!(master_key.as_ref().unwrap().get_signature(),
                   keystore.get_key().get_signature());
    }
}

#[test]
fn test_keystore_open_with_added_key() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized KeyStore.
    fs::remove_file(path.as_path()).unwrap();

    let keya = NormalKey::new_random().unwrap();
    let keyb = NormalKey::new_random().unwrap();
    assert_ne!(keya.get_signature(), keyb.get_signature());
    let master_key: Option<NormalKey>;

    {
        let mut keystore = KeyStore::open_or_new(path.as_path(), &keya).unwrap();
        master_key = Some(keystore.get_key().clone());

        assert!(keystore.add(&keyb).unwrap());
    }

    {
        let keystore = KeyStore::open_or_new(path.as_path(), &keyb).unwrap();
        assert_eq!(master_key.as_ref().unwrap().get_signature(),
                   keystore.get_key().get_signature());
    }
}

#[test]
fn test_add_duplicate_key() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized KeyStore.
    fs::remove_file(path.as_path()).unwrap();

    let wrap_key = NormalKey::new_random().unwrap();
    // Note that creating a new KeyStore automatically adds the given key.
    let mut keystore = KeyStore::open_or_new(path.as_path(), &wrap_key).unwrap();
    // Check that adding the same key again doesn't work.
    assert!(!keystore.add(&wrap_key).unwrap());
}

#[test]
fn test_remove_unused_key() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized KeyStore.
    fs::remove_file(path.as_path()).unwrap();

    let wrap_key = NormalKey::new_random().unwrap();
    let mut keystore = KeyStore::open_or_new(path.as_path(), &wrap_key).unwrap();
    // Test that removing some other key returns false, since it isn't in the
    // KeyStore.
    let other_key = NormalKey::new_random().unwrap();
    assert!(!keystore.remove(&other_key).unwrap());
}

#[test]
fn test_remove_only_key() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_owned();
    // Remove the file: an empty file isn't a valid serialized KeyStore.
    fs::remove_file(path.as_path()).unwrap();

    let key = NormalKey::new_random().unwrap();
    let mut keystore = KeyStore::open_or_new(path.as_path(), &key).unwrap();
    // Test that removing the sole key is treated as an error.
    assert!(keystore.remove(&key).is_err());
}
