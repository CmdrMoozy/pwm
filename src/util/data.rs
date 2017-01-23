// pwm - A simple password manager for Linux.
// Copyright (C) 2015  Axel Rasmussen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use data_encoding::base64;
use ::error::Result;
use sodiumoxide::utils::memzero;
use std::fmt;
use std::fs::File;
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};
use std::str;

/// SensitiveData is, essentially, a vector of bytes which attempts to treat
/// its contents as particularly sensitive. In particular, when Drop'ed a
/// SensitiveData will zero out the memory holding its contents to avoid
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
    /// Load the given string into a new SensitiveData instance. The given
    /// string must be in a format as returned by SensitiveData.to_string(), or
    /// the behavior is undefined (most likely an error will be returned).
    pub fn from_string(s: String) -> Result<SensitiveData> {
        Ok(SensitiveData::from(try!(base64::decode(&s.into_bytes()[..]))))
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
        try!(file.read_to_end(&mut data));
        Ok(SensitiveData::from(data))
    }

    fn as_slice(&self) -> &[u8] { &self.data }

    pub fn len(&self) -> usize { self.data.len() }

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

        SensitiveData { data: data.into_boxed_slice() }
    }
}

impl Drop for SensitiveData {
    fn drop(&mut self) { memzero(&mut self.data); }
}

impl fmt::Display for SensitiveData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", base64::encode(&self.data))
    }
}

impl From<Vec<u8>> for SensitiveData {
    fn from(data: Vec<u8>) -> SensitiveData { SensitiveData { data: data.into_boxed_slice() } }
}

impl Index<usize> for SensitiveData {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 { &self.as_slice()[index] }
}

impl Index<Range<usize>> for SensitiveData {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &[u8] { self.as_slice().index(index) }
}

impl Index<RangeTo<usize>> for SensitiveData {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &[u8] { self.as_slice().index(index) }
}

impl Index<RangeFrom<usize>> for SensitiveData {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &[u8] { self.as_slice().index(index) }
}

impl Index<RangeFull> for SensitiveData {
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &[u8] { self.as_slice().index(index) }
}
