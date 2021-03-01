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
use bdrck::crypto::key::{AbstractKey, Digest, Nonce};

/// PwmKey is a stupid shim which lets us adapt any AbstractKey implementor with an error type we
/// can convert from. This let us write functions that return or accept things which satisfy
/// "AbstractKey<Error = Error>".
pub struct PwmKey<E: Into<Error>, K: AbstractKey<Error = E>>(K);

impl<E: Into<Error>, K: AbstractKey<Error = E>> AbstractKey for PwmKey<E, K> {
    type Error = Error;

    fn get_digest(&self) -> Digest {
        self.0.get_digest()
    }

    fn encrypt(&self, plaintext: &[u8], nonce: Option<Nonce>) -> Result<(Option<Nonce>, Vec<u8>)> {
        self.0.encrypt(plaintext, nonce).map_err(|e| e.into())
    }

    fn decrypt(&self, nonce: Option<&Nonce>, ciphertext: &[u8]) -> Result<Vec<u8>> {
        self.0.decrypt(nonce, ciphertext).map_err(|e| e.into())
    }
}

impl<E: Into<Error>, K: AbstractKey<Error = E>> From<K> for PwmKey<E, K> {
    fn from(k: K) -> Self {
        PwmKey::<E, K>(k)
    }
}
