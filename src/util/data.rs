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
use std::io::Read;
use std::path::Path;

const MAX_KEY_FILE_SIZE_BYTES: u64 = 1024 * 1024 * 10; // 10 MiB

pub type Secret = Vec<u8>;
pub type SecretSlice = [u8];

pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Secret> {
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
    Ok(data)
}

pub fn encode(secret: &SecretSlice) -> String {
    BASE64.encode(secret)
}

pub fn decode(encoded: &str) -> Result<Secret> {
    Ok(match BASE64.decode(encoded.as_bytes()) {
        Ok(data) => data,
        Err(e) => return Err(Error::Base64(e)),
    })
}
