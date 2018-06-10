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

use data_encoding::BASE64;
use error::Result;
use sodiumoxide::utils::memzero;
use std::fs::File;
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};

/// `SensitiveData` is, essentially, a vector of bytes which attempts to treat
/// its contents as particularly sensitive. In particular, when Drop'ed a
/// `SensitiveData` will zero out the memory holding its contents to avoid
/// leaking them. Additionally, the API tries to prevent callers from doing
/// things which would leak its contents.
///
/// However, this is really only "defense-in-depth". Fundamentally, it is not
/// possible to really guarantee that a) the contents will only ever be stored
/// in one place in memory, and that b) they will be zero'ed out when this
/// struct is Drop'ed. Consider, for example, that some or all of the contents
/// of this struct may end up in CPU cache, for example. Also, although in
/// general we want to treat unencrypted data carefully, consider also that the
/// main use case of a password manager is to display unencrypted saved
/// passwords to the user, e.g. on a terminal or even in a GUI.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensitiveData {
    data: Box<[u8]>,
}

impl SensitiveData {
    /// Load the given encoded String into a new SensitiveData instance. The
    /// given String must be in a format as returned by `decode`, or the
    /// behavior of this function is undefined (most likely an error will be
    /// returned).
    pub fn decode(s: String) -> Result<SensitiveData> {
        Ok(SensitiveData::from(BASE64.decode(&s.into_bytes()[..])?))
    }

    /// Return an encoded version of this struct's data as a String. The
    /// returned string is not human-readable, but it is suitable for use
    /// with decode.
    pub fn encode(&self) -> String {
        BASE64.encode(&self.data)
    }

    /// Try to return a String which interprets this structure's bytes as a
    /// UTF8-encoded string. If decoding is not possible, an error is returned
    /// instead.
    fn to_utf8(&self) -> Result<String> {
        Ok(String::from_utf8((&self[..]).to_vec())?)
    }

    /// Return a copy of this structure's data in a format which is suitable
    /// for being displayed to a human. There are several cases being handled
    /// here:
    ///
    /// - If the data is valid UTF-8 encoded character data, it will be
    ///   interpreted as such and returned as a normal string.
    /// - If the data is binary (or force_binary is set), and require_utf8
    ///   is set, then we will return the data as a base64-encoded string.
    /// - Otherwise, None is returned, and the caller can access the raw
    ///   bytes using `as_slice` or similar.
    pub fn display(&self, force_binary: bool, require_utf8: bool) -> Option<String> {
        let as_utf8 = self.to_utf8();
        let is_binary = force_binary || as_utf8.is_err();

        if !is_binary {
            Some(as_utf8.unwrap())
        } else if require_utf8 {
            Some(self.encode())
        } else {
            None
        }
    }

    /// Load the contents of the given file into a new SensitiveData instance.
    ///
    /// The file is read using the Rust standard library's typical file reading
    /// tooling, so it is possible that the contents of the file will be leaked
    /// e.g. in buffers, for example. In general this is not considered too
    /// concerning, because a) the contents of the file are already persisted
    /// to a block device, and b) there is like no sane way to guarantee that
    /// no buffering or copying happens e.g. inside the kernel.
    pub fn from_file(file: &mut File) -> Result<SensitiveData> {
        use std::io::Read;
        let mut data: Vec<u8> = vec![];
        file.read_to_end(&mut data)?;
        Ok(SensitiveData::from(data))
    }

    fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return a new SensitiveData which contains the concatenation of the
    /// contents of this struct and the given other struct.
    pub fn concat(self, other: SensitiveData) -> SensitiveData {
        SensitiveData {
            data: self.data
                .iter()
                .cloned()
                .chain(other.data.iter().cloned())
                .collect::<Vec<u8>>()
                .into_boxed_slice(),
        }
    }

    /// Return a new SensitiveData which contains a copy of this struct's data,
    /// but truncated to the given length. If the given length is longer than
    /// len(), the resulting struct will be an exact duplicate.
    pub fn truncate(self, len: usize) -> SensitiveData {
        let mut data: Vec<u8> = self.data.to_vec();
        if len < data.len() {
            memzero(&mut data[len..]);
        }
        data.truncate(len);

        SensitiveData {
            data: data.into_boxed_slice(),
        }
    }
}

impl Drop for SensitiveData {
    fn drop(&mut self) {
        memzero(&mut self.data);
    }
}

impl From<Vec<u8>> for SensitiveData {
    fn from(data: Vec<u8>) -> SensitiveData {
        SensitiveData {
            data: data.into_boxed_slice(),
        }
    }
}

impl Index<usize> for SensitiveData {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.as_slice()[index]
    }
}

impl Index<Range<usize>> for SensitiveData {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &[u8] {
        self.as_slice().index(index)
    }
}

impl Index<RangeTo<usize>> for SensitiveData {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &[u8] {
        self.as_slice().index(index)
    }
}

impl Index<RangeFrom<usize>> for SensitiveData {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &[u8] {
        self.as_slice().index(index)
    }
}

impl Index<RangeFull> for SensitiveData {
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &[u8] {
        self.as_slice().index(index)
    }
}
