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

use bdrck::testing::temp;
use sodiumoxide::randombytes::randombytes;
use std::fs::File;
use util::data::*;

#[test]
fn test_from_file() {
    use std::io::Write;

    let data: Vec<u8> = Vec::from("Some arbitrary test string.".as_bytes());
    let temp_file = temp::File::new_file().unwrap();

    {
        let mut file = File::create(temp_file.path()).unwrap();
        file.write_all(data.as_slice()).unwrap();
    }

    let mut file = File::open(temp_file.path()).unwrap();
    let loaded_data = SensitiveData::from_file(&mut file).unwrap();
    assert_eq!(data.len(), loaded_data.len());
    assert_eq!(data.as_slice(), &loaded_data[..]);
}

#[test]
fn test_concat() {
    let data: Vec<u8> = Vec::from("Some arbitrary test string.".as_bytes());
    let (data_a, data_b) = data.as_slice().split_at(data.len() / 2);
    assert!(data_a.len() > 0 && data_b.len() > 0);

    let sensitive_data_a = SensitiveData::from(data_a.to_vec());
    let sensitive_data_b = SensitiveData::from(data_b.to_vec());
    let sensitive_data = sensitive_data_a.concat(sensitive_data_b);

    assert_eq!(data.len(), sensitive_data.len());
    assert_eq!(data.as_slice(), &sensitive_data[..]);
}

#[test]
fn test_truncate() {
    let mut data: Vec<u8> = Vec::from("Some arbitrary test string.".as_bytes());
    let truncated_len = data.len() / 2;
    assert!(truncated_len > 0);
    let sensitive_data = SensitiveData::from(data.clone()).truncate(truncated_len);
    data.truncate(truncated_len);

    assert_eq!(data.len(), sensitive_data.len());
    assert_eq!(data.as_slice(), &sensitive_data[..]);
}

#[test]
fn test_decode() {
    let src: String = "Some arbitrary test string.".to_owned();
    let sd_from_bytes = SensitiveData::from(src.as_bytes().to_vec());
    let sd_to_string = sd_from_bytes.encode();
    assert_ne!(src, sd_to_string);
    let sd_from_string = SensitiveData::decode(sd_to_string).unwrap();
    assert_eq!(src.as_bytes(), &sd_from_string[..]);
}

#[test]
fn test_encode() {
    let data = SensitiveData::from(randombytes(1024));
    assert!(data.encode().len() > 0);
}

#[test]
fn test_indexing() {
    let data: Vec<u8> = Vec::from("Some arbitrary test string.".as_bytes());
    let sensitive_data = SensitiveData::from(data.clone());

    assert_eq!(data[11], sensitive_data[11]);
    assert_eq!(&data[5..16], &sensitive_data[5..16]);
    assert_eq!(&data[5..], &sensitive_data[5..]);
    assert_eq!(&data[..16], &sensitive_data[..16]);
    assert_eq!(&data[..], &sensitive_data[..]);
}
