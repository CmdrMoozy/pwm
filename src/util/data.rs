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

use ::error::Result;
use sodiumoxide::utils::memzero;
use std::fmt;
use std::fs::File;
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};
use std::path::Path;
use std::str;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensitiveData {
    data: Box<[u8]>,
}

impl SensitiveData {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<SensitiveData> {
        use std::io::Read;
        let mut file = try!(File::open(path));
        let mut data: Vec<u8> = vec![];
        try!(file.read_to_end(&mut data));
        Ok(SensitiveData::from(data))
    }

    fn as_slice(&self) -> &[u8] { &self.data }

    pub fn len(&self) -> usize { self.data.len() }

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
        write!(f,
               "{}",
               try!(str::from_utf8(&self.data).or(Err(fmt::Error {}))))
    }
}

impl From<Vec<u8>> for SensitiveData {
    fn from(data: Vec<u8>) -> SensitiveData { SensitiveData { data: data.into_boxed_slice() } }
}

impl From<String> for SensitiveData {
    fn from(data: String) -> SensitiveData { SensitiveData::from(Vec::from(data)) }
}

impl<'a> From<&'a str> for SensitiveData {
    fn from(data: &'a str) -> SensitiveData { SensitiveData::from(Vec::from(data)) }
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
