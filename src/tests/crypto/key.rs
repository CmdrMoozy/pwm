use crypto::configuration::Configuration;
use crypto::key::*;
use error::*;
use sodiumoxide::crypto::pwhash::Salt;
use sodiumoxide::randombytes::randombytes;
use util::data::SensitiveData;

fn custom_salt_configuration(salt: Salt) -> Configuration {
    let default = Configuration::default();
    Configuration::new(salt, default.get_mem_limit(), default.get_ops_limit())
}

fn new_password_key(password: &str, salt: Option<Salt>) -> Result<NormalKey> {
    NormalKey::new_password(SensitiveData::from(password.as_bytes().to_vec()),
                            salt.map(|salt| custom_salt_configuration(salt)))
}

#[test]
fn test_random_key_generation() { let _key = NormalKey::new_random().unwrap(); }

#[test]
fn test_password_key_derivation() {
    let _key = new_password_key("foobar",
                                Some(Salt::from_slice(&randombytes(32)[..]).unwrap()))
        .unwrap();
}

#[test]
fn test_basic_key_signature_comparison() {
    let a = NormalKey::new_random().unwrap();
    let b = NormalKey::new_random().unwrap();
    let c = a.clone();

    assert_eq!(a.get_signature(), c.get_signature());
    assert_ne!(a.get_signature(), b.get_signature());
}

#[test]
fn test_encryption_roundtrip() {
    let key = new_password_key("foobar", None).unwrap();
    let plaintext = SensitiveData::from(randombytes(1024));
    let (nonce, ciphertext) = key.encrypt(plaintext.clone());
    let decrypted = key.decrypt(ciphertext.as_slice(), &nonce).unwrap();
    assert_eq!(plaintext, decrypted);
}

#[test]
fn test_decrypting_with_wrong_key_fails() {
    let key = new_password_key("foobar", None).unwrap();
    let plaintext = SensitiveData::from(randombytes(1024));
    let (nonce, ciphertext) = key.encrypt(plaintext);

    let wrong_key = new_password_key("raboof", None).unwrap();
    let decrypted_result = wrong_key.decrypt(ciphertext.as_slice(), &nonce);
    assert!(decrypted_result.is_err());
}

#[test]
fn test_wrapping_roundtrip() {
    let a = NormalKey::new_random().unwrap();
    let b = NormalKey::new_random().unwrap();
    let wrapped = a.clone().wrap(&b).unwrap();
    assert_eq!(wrapped.get_signature(), b.get_signature());
    let unwrapped = wrapped.unwrap(&b).unwrap();
    assert_eq!(unwrapped.get_signature(), a.get_signature());
}

#[test]
fn test_unwrapping_with_wrong_key_fails() {
    let a = NormalKey::new_random().unwrap();
    let b = NormalKey::new_random().unwrap();
    let wrapped = a.clone().wrap(&b).unwrap();
    assert!(wrapped.unwrap(&a).is_err());
}
