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

use crate::secret::*;
use bdrck::testing::temp;
use std::fs::File;

#[test]
fn test_from_file() {
    use std::io::Write;

    let data: Vec<u8> = Vec::from("Some arbitrary test string.".as_bytes());
    let temp_file = temp::File::new_file().unwrap();

    {
        let mut file = File::create(temp_file.path()).unwrap();
        file.write_all(data.as_slice()).unwrap();
    }

    let loaded_data = Secret::load_file(temp_file.path()).unwrap();
    assert_eq!(data.len(), loaded_data.len());
    assert_eq!(data.as_slice(), &loaded_data.as_slice()[..]);
}

#[test]
fn test_encode_decode_round_trip() {
    let original = "Some arbitrary test string.";
    let original_data: Secret = original.to_owned().into();
    let encoded = original_data.encode();
    assert_ne!(original, encoded);
    let decoded = Secret::decode(&encoded).unwrap();
    assert!(original_data == decoded);
}
