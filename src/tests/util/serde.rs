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

use util::serde::*;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct TestStruct {
    a: String,
    b: u64,
    c: bool,
}

#[test]
fn test_binary_round_trip() {
    let original = TestStruct {
        a: "this is a test!".to_owned(),
        b: 42,
        c: true,
    };

    let serialized = serialize_binary(&original).unwrap();
    let deserialized = deserialize_binary(serialized.as_slice()).unwrap();
    assert_eq!(original, deserialized);
}
