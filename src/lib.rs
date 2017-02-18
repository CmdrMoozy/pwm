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

extern crate bdrck_config;
extern crate bdrck_log;
extern crate bincode;
extern crate byteorder;
#[cfg(feature = "clipboard")]
extern crate clipboard;
extern crate data_encoding;
#[macro_use]
extern crate error_chain;
extern crate git2;
extern crate isatty;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rpassword;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sodiumoxide;

pub mod configuration;
pub mod crypto;
pub mod error;
pub mod repository;
pub mod util;

#[cfg(test)]
mod tests;

pub fn init() -> ::error::Result<()> {
    if !sodiumoxide::init() {
        bail!("sodiumoxide initialization failed");
    }

    Ok(())
}
