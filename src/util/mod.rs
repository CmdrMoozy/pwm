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

#[cfg(feature = "clipboard")]
pub mod clipboard;
pub mod data;
pub mod git;
pub mod lazy;

use crate::error::Result;
use bdrck;
use std::io;

/// Prompt the user for a password using the given prompt on stderr, and then
/// read the result on stdin. If confirm is set, we'll prompt for the password
/// twice, and make sure they copies match.
pub fn password_prompt(prompt: &str, confirm: bool) -> Result<data::Secret> {
    Ok(match confirm {
        false => bdrck::cli::prompt_for_string(bdrck::cli::Stream::Stderr, prompt, true)?,
        true => bdrck::cli::prompt_for_string_confirm(bdrck::cli::Stream::Stderr, prompt, true)?,
    }
    .into())
}

/// Prompt the user for multiple lines of password data using the given prompt
/// on stderr. We'll keep reading lines of text from stdin until we read "EOF".
pub fn multiline_password_prompt(prompt: &str) -> Result<data::Secret> {
    use std::io::Write;

    writeln!(&mut io::stderr(), "{}", prompt)?;

    let stdin = io::stdin();
    let mut secret: data::Secret = vec![];
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer == "EOF\n" {
            break;
        }
        secret.append(&mut buffer.into_bytes());
    }
    let secret_len = secret.len();
    secret.truncate(secret_len - 1);

    Ok(secret)
}
