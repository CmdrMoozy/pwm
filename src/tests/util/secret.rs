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

use crate::util::secret::*;
use bdrck::crypto::secret::Secret;

#[test]
fn test_encode_round_trip() {
    crate::init().unwrap();

    let test_str = "this is a test";
    let mut test_secret = Secret::with_len(test_str.as_bytes().len()).unwrap();
    unsafe { test_secret.as_mut_slice() }.copy_from_slice(test_str.as_bytes());

    let encoded = encode(&test_secret);
    let decoded = decode(&encoded).unwrap();

    assert_eq!(test_secret.len(), decoded.len());
    unsafe {
        assert_eq!(test_secret.as_slice(), decoded.as_slice());
    }
}
