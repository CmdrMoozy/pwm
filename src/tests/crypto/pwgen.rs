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

use crypto::pwgen::*;


fn generate_password_str(
    length: Option<usize>,
    charsets: &[CharacterSet],
    exclude: &[char],
) -> String {
    generate_password(
        length.unwrap_or(RECOMMENDED_MINIMUM_PASSWORD_LENGTH),
        charsets,
        exclude,
    ).unwrap()
        .display(false, false)
        .unwrap()
}

#[test]
fn test_generating_empty_password() {
    assert!(generate_password(0, &[CharacterSet::Letters], &[]).is_err());
}

#[test]
fn test_generating_with_no_character_set() {
    assert!(generate_password(RECOMMENDED_MINIMUM_PASSWORD_LENGTH, &[], &[]).is_err());
}

#[test]
fn test_excluding_characters() {
    let password = generate_password_str(Some(1000), &[CharacterSet::Numbers], &['7']);
    assert!(!password.contains('7'));
}

#[test]
fn test_excluding_all_characters() {
    assert!(
        generate_password(
            RECOMMENDED_MINIMUM_PASSWORD_LENGTH,
            &[CharacterSet::Numbers],
            &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        ).is_err()
    );
}

#[test]
fn test_pwgen_charset() {
    for _ in 0..10 {
        let password = generate_password_str(None, &[CharacterSet::Letters], &[]);
        assert_eq!(RECOMMENDED_MINIMUM_PASSWORD_LENGTH, password.len());
        assert!(
            password
                .chars()
                .map(|c| c.is_alphabetic())
                .fold(true, |acc, isalpha| acc && isalpha)
        );
    }

    for _ in 0..10 {
        let password = generate_password_str(None, &[CharacterSet::Numbers], &[]);
        assert_eq!(RECOMMENDED_MINIMUM_PASSWORD_LENGTH, password.len());
        assert!(
            password
                .chars()
                .map(|c| c.is_digit(10))
                .fold(true, |acc, isdigit| acc && isdigit)
        );
    }
}
