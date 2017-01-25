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

pub mod data;
pub mod git;

use error::Result;
use rpassword;

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
