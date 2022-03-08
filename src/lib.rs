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
    anonymous_parameters,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces
)]
#![warn(bare_trait_objects, unreachable_pub, unused_qualifications)]

pub mod cli;
pub mod configuration;
pub mod crypto;
pub mod error;
pub mod output;
#[cfg(feature = "piv")]
pub mod piv;
pub mod repository;
pub mod secret;
pub mod util;
#[cfg(feature = "wifiqr")]
pub mod wifiqr;

#[cfg(test)]
mod tests;

pub fn init() -> crate::error::Result<()> {
    bdrck::init()?;
    #[cfg(feature = "piv")]
    yubirs::init()?;
    Ok(())
}

pub fn init_with_configuration() -> crate::error::Result<crate::configuration::SingletonHandle> {
    init()?;
    Ok(crate::configuration::SingletonHandle::new(None)?)
}
