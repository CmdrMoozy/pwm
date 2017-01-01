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

use ::error::{Error, Result};
use git2::{ErrorClass, ErrorCode, Repository};
use std::path::Path;

pub fn open_repository<P: AsRef<Path>>(path: P, create: bool) -> Result<Repository> {
    let path = path.as_ref();
    match Repository::open(path) {
        Ok(repository) => Ok(repository),
        Err(error) => {
            match create && error.class() == ErrorClass::Repository &&
                  error.code() == ErrorCode::NotFound {
                false => Err(Error::from(error)),
                true => Ok(try!(Repository::init(path))),
            }
        },
    }
}
