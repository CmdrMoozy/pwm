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

mod impls;
pub(crate) mod util;

use crate::cli;
use crate::error::*;
use flaggy::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use yubirs::piv::id;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyConfiguration {
    /// The name of the SmartCard reader this key is found on. Generally an
    /// explicit value is not needed (the default is fine), assuming only one
    /// SmartCard is plugged into the system at a time.
    pub reader: Option<String>,
    /// The SmartCard slot in which the private key resides.
    pub slot: id::Key,
    /// The path to the public key which matches the private key stored on the
    /// hardware. This must be a file, in PEM format.
    pub public_key: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Configuration {
    /// Keys which, if present, can be used to unlock the master key, in lieu of
    /// using password authentication.
    pub keys: Vec<KeyConfiguration>,
}

pub static SLOT_SPEC: Lazy<Spec> = Lazy::new(|| {
    Spec::required(
        "slot",
        "The slot containing the certificate to use",
        Some('s'),
        Some(&::yubirs::piv::id::Key::KeyManagement.to_string()),
    )
});
pub static ALGORITHM_SPEC: Lazy<Spec> = Lazy::new(|| {
    Spec::required(
        "algorithm",
        "The key algorithm to use",
        Some('a'),
        Some(&::yubirs::piv::id::Algorithm::Rsa2048.to_string()),
    )
});
pub static PIN_POLICY_SPEC: Lazy<Spec> = Lazy::new(|| {
    Spec::required(
        "pin_policy",
        "The PIN verification policy to use for this key",
        None,
        Some(&::yubirs::piv::id::PinPolicy::Default.to_string()),
    )
});
pub static TOUCH_POLICY_SPEC: Lazy<Spec> = Lazy::new(|| {
    Spec::required(
        "touch_policy",
        "The touch policy to use for this key",
        None,
        Some(&::yubirs::piv::id::TouchPolicy::Default.to_string()),
    )
});
pub static OPTIONAL_PUBLIC_KEY_SPEC: Lazy<Spec> = Lazy::new(|| {
    Spec::optional(
    "public_key",
    "The path to write the public key to; only necessary if you want to re-use this key for other applications",
    Some('p'),
)
});
pub static REQUIRED_PUBLIC_KEY_SPEC: Lazy<Spec> = Lazy::new(|| {
    Spec::required(
        "public_key",
        "The path to this PIV device's public key (in PEM format)",
        Some('p'),
        None,
    )
});
pub static READER_SPEC: Lazy<Spec> = Lazy::new(|| {
    Spec::optional(
        "reader",
        "The reader name of the PIV device to remove",
        None,
    )
});
pub static SERIAL_SPEC: Lazy<Spec> = Lazy::new(|| {
    Spec::optional(
        "serial",
        "The serial number of the PIV device to remove",
        None,
    )
});

pub fn build_setuppiv_command() -> Command<'static, Error> {
    Command::new(
        "setuppiv",
        "Set up a PIV device and add it to an existing repository",
        Specs::new(vec![
            cli::REPOSITORY_SPEC.clone(),
            SLOT_SPEC.clone(),
            ALGORITHM_SPEC.clone(),
            PIN_POLICY_SPEC.clone(),
            TOUCH_POLICY_SPEC.clone(),
            OPTIONAL_PUBLIC_KEY_SPEC.clone(),
        ])
        .unwrap(),
        Box::new(impls::setuppiv),
    )
}

pub fn build_addpiv_command() -> Command<'static, Error> {
    Command::new(
        "addpiv",
        "Add an already set up PIV device to an existing repository",
        Specs::new(vec![
            cli::REPOSITORY_SPEC.clone(),
            SLOT_SPEC.clone(),
            REQUIRED_PUBLIC_KEY_SPEC.clone(),
        ])
        .unwrap(),
        Box::new(impls::addpiv),
    )
}

pub fn build_rmpiv_command() -> Command<'static, Error> {
    Command::new(
        "rmpiv",
        "Remove a PIV device key from an existing repository",
        Specs::new(vec![
            cli::REPOSITORY_SPEC.clone(),
            READER_SPEC.clone(),
            SERIAL_SPEC.clone(),
        ])
        .unwrap(),
        Box::new(impls::rmpiv),
    )
}
