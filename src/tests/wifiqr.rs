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

use crate::wifiqr::*;

#[test]
fn test_valid_encoded_output() {
    let out = String::from_utf8(wifiqr_encode("foo", false, "bar".as_bytes()).unwrap()).unwrap();
    assert_eq!("WIFI:S:foo;T:WPA;P:bar;H:false;;", out);

    let out = String::from_utf8(wifiqr_encode("foobar", true, "baz".as_bytes()).unwrap()).unwrap();
    assert_eq!("WIFI:S:foobar;T:WPA;P:baz;H:true;;", out);
}

//pub(crate) fn wifiqr_encode(ssid: String, is_hidden: bool, password: &Secret) -> Result<Secret> {

#[test]
fn test_handles_escaped_characters() {
    let out = String::from_utf8(
        wifiqr_encode(r#"special ";,:\"#, false, r#"\:,;" characters"#.as_bytes()).unwrap(),
    )
    .unwrap();
    assert_eq!(
        "WIFI:S:special \\\"\\;\\,\\:\\\\;T:WPA;P:\\\\\\:\\,\\;\\\" characters;H:false;;",
        out
    );
}
