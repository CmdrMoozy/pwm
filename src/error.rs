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

use backtrace::Backtrace;
use std::cmp::{Eq, PartialEq};
use std::error;
use std::fmt;
use std::io;
use std::result;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Crypto { cause: String },
    Initialization { cause: String },
    Io { cause: String },
    Key { cause: String },
    Padding { cause: String },
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub backtrace: Backtrace,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind: kind,
            backtrace: Backtrace::new(),
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool { self.kind == other.kind }
}

impl Eq for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error { Error::new(ErrorKind::Io { cause: e.to_string() }) }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Crypto { cause: _ } => "Cryptographic error",
            ErrorKind::Initialization { cause: _ } => "Library initialization error",
            ErrorKind::Io { cause: _ } => "Input / output error",
            ErrorKind::Key { cause: _ } => "Key derivation error",
            ErrorKind::Padding { cause: _ } => "Padding / unpadding error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        match self.kind {
            ErrorKind::Crypto { cause: ref c } => {
                f.write_str(format!("{}: {}", self.description(), c).as_str())
            },
            ErrorKind::Initialization { cause: ref c } => {
                f.write_str(format!("{}: {}", self.description(), c).as_str())
            },
            ErrorKind::Io { cause: ref c } => {
                f.write_str(format!("{}: {}", self.description(), c).as_str())
            },
            ErrorKind::Key { cause: ref c } => {
                f.write_str(format!("{}: {}", self.description(), c).as_str())
            },
            ErrorKind::Padding { cause: ref c } => {
                f.write_str(format!("{}: {}", self.description(), c).as_str())
            },
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
