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

use anyhow::{bail, Result};
use bdrck::crypto::secret::Secret;
use data_encoding::BASE64;
use std::fs::File;
use std::path::Path;

const MAX_KEY_FILE_SIZE_BYTES: usize = 1024 * 1024 * 10; // 10 MiB

pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Secret> {
    use std::io::Read;

    let mut file = File::open(path.as_ref())?;
    let len = file.metadata()?.len() as usize;
    if len > MAX_KEY_FILE_SIZE_BYTES {
        bail!(
            "invalid secret file {}; exceeded maximum limit of {} bytes",
            path.as_ref().display(),
            MAX_KEY_FILE_SIZE_BYTES
        );
    }

    let mut s = Secret::with_len(len)?;
    unsafe {
        file.read_exact(s.as_mut_slice())?;
    }
    Ok(s)
}

// TODO: Implement a better migration feature and remove this.
pub fn decode(encoded: &str) -> Result<Secret> {
    let mut s = Secret::with_len(BASE64.decode_len(encoded.len())?)?;

    match BASE64.decode_mut(encoded.as_bytes(), unsafe { s.as_mut_slice() }) {
        Ok(len) => s.resize(len)?,
        Err(e) => bail!("base64 decode error: {:?}", e),
    };

    Ok(s)
}

// TODO: Implement a better migration feature and remove this.
pub fn encode(data: &Secret) -> String {
    BASE64.encode(unsafe { data.as_slice() })
}
