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

pub(crate) mod impls;
pub(crate) mod util;

use crate::cli;
use clap::Args;
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

#[derive(Args, Clone)]
struct SlotArgs {
    #[arg(short = 's', long, default_value_t = id::Key::KeyManagement)]
    /// The slot containing the certificate to use.
    slot: id::Key,
}

#[derive(Args, Clone)]
struct PolicyArgs {
    #[arg(long, default_value_t = id::PinPolicy::Default)]
    /// The PIN verification policy to use for this key.
    pin_policy: id::PinPolicy,

    #[arg(long, default_value_t = id::TouchPolicy::Default)]
    /// The touch verification policy to use for this key.
    touch_policy: id::TouchPolicy,
}

#[derive(Args)]
pub(crate) struct SetupPivArgs {
    #[clap(flatten)]
    repository: cli::RepositoryArgs,
    #[clap(flatten)]
    slot: SlotArgs,

    #[arg(short = 'a', long, default_value_t = id::Algorithm::Rsa2048)]
    /// The key algorithm to use.
    algorithm: id::Algorithm,

    #[clap(flatten)]
    policy: PolicyArgs,

    #[arg(short = 'p', long)]
    /// The path to write the public key to; only necessary if you want to re-use this key for
    /// other applications.
    public_key: Option<PathBuf>,
}

#[derive(Args)]
pub(crate) struct AddPivArgs {
    #[clap(flatten)]
    repository: cli::RepositoryArgs,
    #[clap(flatten)]
    slot: SlotArgs,

    #[arg(short = 'p', long)]
    /// The path to this PIV device's public key (in PEM format).
    public_key: PathBuf,
}

#[derive(Args)]
pub(crate) struct RmPivArgs {
    #[clap(flatten)]
    repository: cli::RepositoryArgs,

    #[arg(long)]
    /// The reader name of the PIV device to remove.
    reader: Option<String>,

    #[arg(long)]
    /// The serial number of the PIV device to remove.
    serial: Option<u32>,
}
