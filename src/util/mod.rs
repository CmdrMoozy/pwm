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

pub mod git;
pub mod lazy;

use crate::error::Result;
use crate::secret::Secret;
use bdrck;
use std::io;

/// Prompt the user for a password using the given prompt on stderr, and then
/// read the result on stdin. If confirm is set, we'll prompt for the password
/// twice, and make sure they copies match.
pub fn password_prompt(prompt: &str, confirm: bool) -> Result<Secret> {
    let sb = match confirm {
        false => bdrck::cli::prompt_for_string(
            bdrck::cli::Stream::Stdin,
            bdrck::cli::Stream::Stderr,
            prompt,
            /*is_sensitive=*/ true,
        )?,
        true => bdrck::cli::prompt_for_string_confirm(
            bdrck::cli::Stream::Stdin,
            bdrck::cli::Stream::Stderr,
            prompt,
            /*is_sensitive=*/ true,
        )?,
    }
    .into_bytes();

    let mut result = Secret::with_len(sb.len());
    result.as_mut_slice().copy_from_slice(sb.as_slice());
    Ok(result)
}

/// A wrapper around `password_prompt`, which will skip the prompt if a
/// "hard-coded" password was provided instead.
pub fn unwrap_password_or_prompt(
    password: Option<Secret>,
    prompt: &str,
    confirm: bool,
) -> Result<Secret> {
    Ok(if let Some(p) = password {
        p
    } else {
        password_prompt(prompt, confirm)?
    })
}

/// Prompt the user for multiple lines of password data using the given prompt
/// on stderr. We'll keep reading lines of text from stdin until we read "EOF".
pub fn multiline_password_prompt(prompt: &str) -> Result<Secret> {
    // TODO: Completely rewrite all of our prompt functions to read directly into Secrets.

    use std::io::Write;

    writeln!(&mut io::stderr(), "{}", prompt)?;

    let stdin = io::stdin();
    let mut secret = Secret::new();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer == "EOF\n" {
            break;
        }

        let old_len = secret.len();
        let new_len = old_len + buffer.len();
        secret.resize(new_len);
        secret.as_mut_slice()[old_len..].copy_from_slice(buffer.into_bytes().as_slice());
    }
    let secret_len = secret.len();
    secret.resize(secret_len - 1);

    Ok(secret)
}
