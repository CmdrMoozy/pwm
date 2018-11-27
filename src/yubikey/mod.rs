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

pub mod command;

use std::path::PathBuf;
use yubirs::piv::id;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyConfiguration {
    /// The name of the SmartCard reader this key is found on. Generally an
    /// explicit value is not needed (the default is fine), assuming only one
    /// SmartCard is plugged into the system at a time, and it is a YubiKey.
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
