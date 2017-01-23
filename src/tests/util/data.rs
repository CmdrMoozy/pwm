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

use sodiumoxide::randombytes::randombytes;
use std::io::SeekFrom;
use ::tests::tempfile::tempfile;
use ::util::data::*;

#[test]
fn test_from_file() {
    use std::io::{Seek, Write};

    let data: Vec<u8> = Vec::from("Some arbitrary test string.".as_bytes());
    let mut file = tempfile().unwrap();

    file.write_all(data.as_slice()).unwrap();
    file.flush().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

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
fn test_from_string() {
    let src: String = "Some arbitrary test string.".to_owned();
    let sd_from_bytes = SensitiveData::from(src.as_bytes().to_vec());
    let sd_to_string = sd_from_bytes.to_string();
    assert_ne!(src, sd_to_string);
    let sd_from_string = SensitiveData::from_string(sd_to_string).unwrap();
    assert_eq!(src.as_bytes(), &sd_from_string[..]);
}

#[test]
fn test_to_string() {
    let data = SensitiveData::from(randombytes(1024));
    assert!(data.to_string().len() > 0);
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
