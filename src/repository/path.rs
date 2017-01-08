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
use std::path::Path as StdPath;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Path {
    relative_path: PathBuf,
    absolute_path: PathBuf,
}

impl Path {
    pub fn new<P: AsRef<StdPath>>(repository: &Repository, path: P) -> Result<Path> {
        let mut absolute_path = PathBuf::from(try!(repository.workdir()));
        absolute_path.push(path.as_ref());
        Ok(Path {
            relative_path: PathBuf::from(path.as_ref()),
            absolute_path: absolute_path,
        })
    }

    pub fn relative_path(&self) -> &StdPath { self.relative_path.as_path() }

    pub fn absolute_path(&self) -> &StdPath { self.absolute_path.as_path() }
}
