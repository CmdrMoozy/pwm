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

use bincode::{self, deserialize, serialize};
use error::Result;
use serde::{Deserialize, Serialize};

pub fn serialize_binary<T: Serialize>(data: &T) -> Result<Vec<u8>> {
    match serialize(data, bincode::Infinite) {
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
