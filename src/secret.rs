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
use data_encoding::BASE64;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const MAX_KEY_FILE_SIZE_BYTES: u64 = 1024 * 1024 * 10; // 10 MiB

#[derive(Clone, PartialEq)]
pub struct Secret {
    inner: Vec<u8>,
}

impl Secret {
    pub fn new() -> Self {
        Secret { inner: Vec::new() }
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path.as_ref())?;
        if file.metadata()?.len() > MAX_KEY_FILE_SIZE_BYTES {
            return Err(Error::InvalidArgument(format!(
                "invalid secret file {}; exceeded maximum limit of {} bytes",
                path.as_ref().display(),
                MAX_KEY_FILE_SIZE_BYTES
            )));
        }
        let mut data: Vec<u8> = vec![];
        file.read_to_end(&mut data)?;
        Ok(Secret { inner: data })
    }

    pub fn decode(encoded: &str) -> Result<Self> {
        Ok(match BASE64.decode(encoded.as_bytes()) {
            Ok(data) => Secret { inner: data },
            Err(e) => return Err(Error::Base64(e)),
        })
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn resize(&mut self, len: usize) {
        self.inner.resize(len, 0)
    }

    pub fn append(&mut self, other: &mut Vec<u8>) {
        self.inner.append(other)
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    pub fn encode(&self) -> String {
        BASE64.encode(&self.inner)
    }
}

impl From<String> for Secret {
    fn from(s: String) -> Self {
        Secret { inner: s.into() }
    }
}

impl From<Vec<u8>> for Secret {
    fn from(v: Vec<u8>) -> Self {
        Secret { inner: v }
    }
}

impl Write for Secret {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
