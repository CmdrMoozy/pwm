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

use crate::crypto::padding::*;
use crate::secret::Secret;
use crate::tests::random_secret;

#[test]
fn test_padding_round_trip() {
    let mut data = random_secret(123);
    let original_data = data.try_clone().unwrap();
    pad(&mut data);
    assert!(data.len() > original_data.len());
    unpad(&mut data).unwrap();
    assert!(original_data.as_slice() == data.as_slice());
}

#[test]
fn test_unpadding_invalid_size() {
    let mut data = Secret::new();
    assert!(unpad(&mut data).is_err());
}
