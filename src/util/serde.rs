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

use bincode::{deserialize, serialize};
use bincode::SizeLimit;
use error::Result;
use serde::{Deserialize, Serialize};

pub fn serialize_binary<T: Serialize>(data: &T) -> Result<Vec<u8>> {
    match serialize(data, SizeLimit::Infinite) {
        Err(e) => bail!("Binary serialization failed: {}", e),
        Ok(s) => Ok(s),
    }
}

pub fn deserialize_binary<T: Deserialize>(data: &[u8]) -> Result<T> {
    match deserialize(data) {
        Err(e) => bail!("Binary deserialization failed: {}", e),
        Ok(d) => Ok(d),
    }
}
