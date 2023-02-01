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

use anyhow::Error;
use bdrck::crypto::digest::Digest;
use bdrck::crypto::key::{AbstractKey, Nonce};
use bdrck::crypto::secret::Secret;
use std::fmt;
use std::result::Result as StdResult;

#[derive(Debug)]
pub struct KeyError(Error);

impl From<Error> for KeyError {
    fn from(e: Error) -> Self {
        KeyError(e)
    }
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for KeyError {}

impl KeyError {
    pub fn into_inner(self) -> Error {
        self.0
    }
}

pub type KeyResult<T> = StdResult<T, KeyError>;

/// The basic problem we have is, we want to have a function which returns any of various kinds of
/// keys (which may have different error types). So we want a single AbstractKey implementation,
/// which can *wrap* any of those various key kinds, and provide a standard error type for all of
/// them.
///
/// A further problem is, we can't just use the normal Error we use everywhere else for this,
/// because anyhow's Error doesn't implement std::error::Error. So we additionally (above)
/// introduce a new stupid KeyError wrapper which implements std::error::Error.
pub struct PwmKey<E: Into<Error>, K: AbstractKey<Error = E>>(K);

impl<E: Into<Error>, K: AbstractKey<Error = E>> AbstractKey for PwmKey<E, K> {
    type Error = KeyError;

    fn get_digest(&self) -> Digest {
        self.0.get_digest()
    }

    fn serialize(&self) -> KeyResult<Secret> {
        self.0.serialize().map_err(|e| KeyError::from(e.into()))
    }

    fn deserialize(data: Secret) -> KeyResult<Self> {
        K::deserialize(data)
            .map(|k| Self::from(k))
            .map_err(|e| KeyError::from(e.into()))
    }

    fn encrypt(
        &self,
        plaintext: &Secret,
        nonce: Option<Nonce>,
    ) -> KeyResult<(Option<Nonce>, Vec<u8>)> {
        self.0
            .encrypt(plaintext, nonce)
            .map_err(|e| KeyError::from(e.into()))
    }

    fn decrypt(&self, nonce: Option<&Nonce>, ciphertext: &[u8]) -> KeyResult<Secret> {
        self.0
            .decrypt(nonce, ciphertext)
            .map_err(|e| KeyError::from(e.into()))
    }
}

impl<E: Into<Error>, K: AbstractKey<Error = E>> From<K> for PwmKey<E, K> {
    fn from(k: K) -> Self {
        PwmKey::<E, K>(k)
    }
}
