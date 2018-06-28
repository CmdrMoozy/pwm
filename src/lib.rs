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

#![deny(
    anonymous_parameters, trivial_casts, trivial_numeric_casts, unused_extern_crates,
    unused_import_braces
)]
#![warn(bare_trait_objects, unreachable_pub, unused_qualifications)]

extern crate bdrck;
extern crate byteorder;
#[cfg(feature = "clipboard")]
extern crate clipboard;
extern crate data_encoding;
#[macro_use]
extern crate failure;
extern crate git2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rand;
extern crate rmp_serde as msgpack;
extern crate rpassword;
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
    bdrck::init()?;
    Ok(())
}
