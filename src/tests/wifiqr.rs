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

use crate::tests::str_secret;
use crate::wifiqr::*;

fn test_encode(ssid: &str, is_hidden: bool, password: &str) -> String {
    let password = str_secret(password);
    let encoded = wifiqr_encode(ssid, is_hidden, &password).unwrap();
    std::str::from_utf8(encoded.as_slice()).unwrap().to_owned()
}

#[test]
fn test_valid_encoded_output() {
    assert_eq!(
        "WIFI:S:foo;T:WPA;P:bar;H:false;;",
        test_encode("foo", false, "bar")
    );
    assert_eq!(
        "WIFI:S:foobar;T:WPA;P:baz;H:true;;",
        test_encode("foobar", true, "baz")
    );
}

#[test]
fn test_handles_escaped_characters() {
    assert_eq!(
        "WIFI:S:special \\\"\\;\\,\\:\\\\;T:WPA;P:\\\\\\:\\,\\;\\\" characters;H:false;;",
        test_encode(r#"special ";,:\"#, false, r#"\:,;" characters"#)
    );
}
