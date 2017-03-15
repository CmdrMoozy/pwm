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

#[cfg(feature = "clipboard")]
pub mod clipboard;
pub mod data;
pub mod git;
pub mod serde;

use error::Result;
use rpassword;
use std::io;

/// Prompt the user for a password using the given prompt on stderr, and then
/// read the result on stdin. If confirm is set, we'll prompt for the password
/// twice, and make sure they copies match.
pub fn password_prompt(prompt: &str, confirm: bool) -> Result<data::SensitiveData> {
    loop {
        let password = data::SensitiveData::from(try!(rpassword::prompt_password_stderr(prompt))
            .into_bytes());
        if !confirm ||
           data::SensitiveData::from(try!(rpassword::prompt_password_stderr("Confirm: "))
            .into_bytes()) == password {
            return Ok(password);
        }
    }
}

/// Prompt the user for multiple lines of password data using the given prompt
/// on stderr. We'll keep reading lines of text from stdin until we read "EOF".
pub fn multiline_password_prompt(prompt: &str) -> Result<data::SensitiveData> {
    use std::io::Write;

    try!(writeln!(&mut io::stderr(), "{}", prompt));

    let stdin = io::stdin();
    let mut data: Vec<u8> = vec![];
    loop {
        let mut buffer = String::new();
        try!(stdin.read_line(&mut buffer));
        if buffer == "EOF\n" {
            break;
        }
        data.append(&mut buffer.into_bytes());
    }
    let data_len = data.len();
    data.truncate(data_len - 1);

    Ok(data::SensitiveData::from(data))
}
