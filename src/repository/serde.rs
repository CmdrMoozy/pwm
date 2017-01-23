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

use ::error::Result;
use ::repository::Repository;
use serde_json::to_string_pretty;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contents {
    pub contents: HashMap<String, String>,
}

pub fn export(repository: &Repository) -> Result<Contents> {
    let mut contents: Contents = Contents { contents: HashMap::new() };

    for path in try!(repository.list(None)) {
        let plaintext: String = try!(repository.read_decrypt(&path)).to_string();
        contents.contents.insert(try!(path.to_str()).to_owned(), plaintext);
    }

    Ok(contents)
}

pub fn export_serialize(repository: &Repository) -> Result<String> {
    Ok(try!(to_string_pretty(&try!(export(repository)))))
}
