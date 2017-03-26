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
extern crate rand;
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
