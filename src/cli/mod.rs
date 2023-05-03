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

use crate::crypto::pwgen::{CharacterSet, RECOMMENDED_MINIMUM_PASSWORD_LENGTH};
use crate::output::OutputMethod;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Args, Clone)]
pub(crate) struct RepositoryArgs {
    #[arg(short = 'r', long)]
    /// The path to the pwm repository to use.
    pub(crate) repository: Option<PathBuf>,
}

#[derive(Args)]
struct PathArgs {
    /// The saved password path, relative to the repository's root.
    path: String,
}

#[derive(Args)]
pub(crate) struct GenerateArgs {
    #[arg(short = 'A', long)]
    /// Exclude letters from the password.
    pub(crate) exclude_letters: bool,

    #[arg(short = 'N', long)]
    /// Exclude numbers from the password.
    pub(crate) exclude_numbers: bool,

    #[arg(short = 'S', long)]
    /// Include symbols in the password.
    pub(crate) exclude_symbols: bool,

    #[arg(short = 'X', long)]
    /// Exclude a custom set of characters.
    pub(crate) custom_exclude: Option<String>,
}

impl GenerateArgs {
    pub(crate) fn to_charsets(&self) -> Vec<CharacterSet> {
        let mut charsets = Vec::new();
        if !self.exclude_letters {
            charsets.push(CharacterSet::Letters);
        }
        if !self.exclude_numbers {
            charsets.push(CharacterSet::Numbers);
        }
        if !self.exclude_symbols {
            charsets.push(CharacterSet::Symbols);
        }
        charsets
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Get or set a configuration value.
    Config {
        #[arg(short = 'k', long)]
        /// The specific key to get or set.
        key: Option<String>,
        #[arg(short = 's', long)]
        /// The new value to set the key to.
        set: Option<String>,
    },

    /// Initialize a new pwm repository.
    Init {
        #[command(flatten)]
        repository: RepositoryArgs,
    },

    /// Add a new master key to an existing repository.
    AddKey {
        #[command(flatten)]
        repository: RepositoryArgs,
    },

    /// Remove an existing master key from an existing repository.
    RmKey {
        #[command(flatten)]
        repository: RepositoryArgs,
    },

    #[cfg(feature = "piv")]
    /// Set up a PIV device and add it to an existing repository.
    SetupPiv(crate::piv::SetupPivArgs),

    #[cfg(feature = "piv")]
    /// Add an already set up PIV device to an existing repository.
    AddPiv(crate::piv::AddPivArgs),

    #[cfg(feature = "piv")]
    /// Remove a PIV device key from an existing repository.
    RmPiv(crate::piv::RmPivArgs),

    /// List passwords stored in a pwm repository.
    Ls {
        #[command(flatten)]
        repository: RepositoryArgs,

        #[arg(default_value = "")]
        /// The saved password path prefix, relative to the repository's root.
        path_prefix: String,
    },

    /// Retrieve a password or key from a pwm repository.
    Get {
        #[command(flatten)]
        repository: RepositoryArgs,

        #[arg(short = 'b', long)]
        /// Treat the saved password or key as binary data.
        binary: bool,

        #[arg(value_enum, short = 'o', long)]
        /// How to output the retrieved secret.
        output_method: OutputMethod,

        #[command(flatten)]
        path: PathArgs,
    },

    /// Store a password or key in a pwm repository.
    Set {
        #[command(flatten)]
        repository: RepositoryArgs,

        #[arg(short = 'k', long)]
        /// Store a key file instead of a password.
        key_file: Option<PathBuf>,

        #[arg(short = 'm', long)]
        /// Read multiple lines of input data, until 'EOF'.
        multiline: bool,

        #[command(flatten)]
        path: PathArgs,
    },

    /// Remove a password or key from a pwm repository.
    Rm {
        #[command(flatten)]
        repository: RepositoryArgs,

        #[command(flatten)]
        path: PathArgs,
    },

    /// Generate a random password.
    Generate {
        #[arg(short = 'l', long, default_value_t = RECOMMENDED_MINIMUM_PASSWORD_LENGTH)]
        /// The length of the password to generate.
        password_length: usize,

        #[command(flatten)]
        args: GenerateArgs,
    },

    #[cfg(feature = "wifiqr")]
    /// Generate a WiFi password, and render it as a QR code for sharing.
    Wifiqr(crate::wifiqr::WifiqrArgs),

    /// Export all stored passwords as plaintext JSON for backup purposes.
    Export {
        #[command(flatten)]
        repository: RepositoryArgs,
    },

    /// Import stored passwords previously 'export'ed.
    Import {
        #[command(flatten)]
        repository: RepositoryArgs,

        #[arg(short = 'i', long)]
        /// The input file to import from.
        input: PathBuf,
    },
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn execute_command(self) -> Result<()> {
        match self.command {
            Commands::Config { key, set } => impls::config(key, set),
            Commands::Init { repository } => impls::init(repository.repository),
            Commands::AddKey { repository } => impls::addkey(repository.repository),
            Commands::RmKey { repository } => impls::rmkey(repository.repository),
            #[cfg(feature = "piv")]
            Commands::SetupPiv(args) => crate::piv::impls::setuppiv(args),
            #[cfg(feature = "piv")]
            Commands::AddPiv(args) => crate::piv::impls::addpiv(args),
            #[cfg(feature = "piv")]
            Commands::RmPiv(args) => crate::piv::impls::rmpiv(args),
            Commands::Ls {
                repository,
                path_prefix,
            } => impls::ls(repository.repository, path_prefix),
            Commands::Get {
                repository,
                binary,
                output_method,
                path,
            } => impls::get(repository.repository, binary, output_method, path.path),
            Commands::Set {
                repository,
                key_file,
                multiline,
                path,
            } => impls::set(repository.repository, key_file, multiline, path.path),
            Commands::Rm { repository, path } => impls::rm(repository.repository, path.path),
            Commands::Generate {
                password_length,
                args,
            } => impls::generate(password_length, args),
            #[cfg(feature = "wifiqr")]
            Commands::Wifiqr(args) => crate::wifiqr::wifiqr_command(args),
            Commands::Export { repository } => impls::export(repository.repository),
            Commands::Import { repository, input } => impls::import(repository.repository, input),
        }
    }
}
