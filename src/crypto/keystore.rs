use crypto::key::{Key, NormalKey, WrappedKey};
use error::Result;
use sodiumoxide::crypto::secretbox;
use std::fs::File;
use std::path::{Path, PathBuf};
use util::data::SensitiveData;
use util::serde::{deserialize_binary, serialize_binary};

lazy_static! {
    /// This token is used to verify that authentication was successful. We encrypt it with a master
    /// key which we then wrap with user key(s), so we can verify that the user presented a valid
    /// key by trying to decrypt this token.
    static ref AUTH_TOKEN_CONTENTS: Vec<u8> =
        "3c017f717b39247c351154a41d2850e4187284da4b928f13c723d54440ba2dfe".bytes().collect();
}

#[derive(Deserialize, Serialize)]
struct EncryptedContents {
    pub token_nonce: secretbox::Nonce,
    pub token: Vec<u8>,
    pub wrapped_keys: Vec<WrappedKey>,
}

impl EncryptedContents {
    pub fn new(master_key: &NormalKey) -> Result<EncryptedContents> {
        let (nonce, encrypted) =
            master_key.encrypt(SensitiveData::from(AUTH_TOKEN_CONTENTS.clone()));
        Ok(EncryptedContents {
            token_nonce: nonce,
            token: encrypted,
            wrapped_keys: Vec::new(),
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<EncryptedContents> {
        use std::io::Read;
        let mut file = try!(File::open(path));
        let mut contents: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut contents));
        deserialize_binary(contents.as_slice())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        use std::io::Write;
        let data = try!(serialize_binary(self));
        let mut file = try!(File::create(path));
        Ok(try!(file.write_all(data.as_slice())))
    }

    pub fn is_master_key(&self, key: &NormalKey) -> Result<bool> {
        let decrypted = try!(key.decrypt(self.token.as_slice(), &self.token_nonce));
        Ok(&decrypted[..] == AUTH_TOKEN_CONTENTS.as_slice())
    }

    /// Add the given wrapped key to this data structure. Note that it is up to
    /// the caller to ensure that the proper global "master key" has been
    /// wrapped before passing it to this function. Returns true if the key was
    /// added, or false if an existing key with a matching signature was found.
    pub fn add(&mut self, wrapped_key: WrappedKey) -> bool {
        if self.wrapped_keys
            .iter()
            .filter(|k| k.get_signature() == wrapped_key.get_signature())
            .count() > 0 {
            return false;
        }
        self.wrapped_keys.push(wrapped_key);
        true
    }

    /// Remove any keys wrapped with the given key from this data structure.
    /// Returns true if any keys were removed, or false if no keys wrapped with
    /// the given key could be found.
    pub fn remove(&mut self, wrap_key: &NormalKey) -> bool {
        let original_length = self.wrapped_keys.len();
        let wrapped_keys = self.wrapped_keys
            .drain(..)
            .filter(|k| k.get_signature() != wrap_key.get_signature())
            .collect();
        self.wrapped_keys = wrapped_keys;
        self.wrapped_keys.len() != original_length
    }
}

pub struct KeyStore {
    path: PathBuf,
    master_key: NormalKey,
    encrypted_contents: EncryptedContents,
}

impl KeyStore {
    fn new<P: AsRef<Path>>(path: P) -> Result<KeyStore> {
        let master_key = try!(NormalKey::new_random());
        let encrypted_contents = try!(EncryptedContents::new(&master_key));

        Ok(KeyStore {
            path: PathBuf::from(path.as_ref()),
            master_key: master_key,
            encrypted_contents: encrypted_contents,
        })
    }

    fn open<P: AsRef<Path>>(path: P, key: &NormalKey) -> Result<KeyStore> {
        let contents = try!(EncryptedContents::open(path.as_ref()));
        let mut master_key: Option<NormalKey> = None;
        for wrapped_key in contents.wrapped_keys.iter() {
            if let Ok(unwrapped_key) = wrapped_key.unwrap(key) {
                if try!(contents.is_master_key(&unwrapped_key)) {
                    master_key = Some(unwrapped_key);
                }
            }
        }

        if master_key.is_some() {
            return Ok(KeyStore {
                path: PathBuf::from(path.as_ref()),
                master_key: master_key.unwrap(),
                encrypted_contents: contents,
            });
        }
        bail!("Failed to unwrap master key with the provided wrapping key.");
    }

    pub fn open_or_new<P: AsRef<Path>>(path: P, key: &NormalKey) -> Result<KeyStore> {
        if path.as_ref().exists() {
            Self::open(path, key)
        } else {
            let mut keystore = try!(Self::new(path));
            try!(keystore.add(key));
            Ok(keystore)
        }
    }

    pub fn get_key(&self) -> &NormalKey { &self.master_key }

    pub fn add(&mut self, key: &NormalKey) -> Result<bool> {
        Ok(self.encrypted_contents.add(try!(self.master_key.clone().wrap(key))))
    }

    pub fn remove(&mut self, key: &NormalKey) -> bool { self.encrypted_contents.remove(key) }
}

impl Drop for KeyStore {
    fn drop(&mut self) { self.encrypted_contents.save(self.path.as_path()).unwrap(); }
}
