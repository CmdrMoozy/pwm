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

use ::crypto::padding::*;
use sodiumoxide::randombytes::randombytes;
use util::data::SensitiveData;

#[test]
fn test_padding_round_trip() {
    let mut data = SensitiveData::from(randombytes(123));
    let original_data = data.clone();
    data = pad(data);
    assert!(data.len() > original_data.len());
    data = unpad(data).unwrap();
    assert_eq!(original_data, data);
}

#[test]
fn test_unpadding_invalid_size() {
    assert!(unpad(SensitiveData::from(vec![])).is_err());
}
