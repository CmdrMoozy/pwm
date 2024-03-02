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

use crate::cli::GenerateArgs;
use crate::crypto::pwgen;
use crate::output::{output_secret, InputEncoding, OutputMethod};
use crate::util::password_prompt;
use anyhow::{bail, Result};
use bdrck::crypto::secret::Secret;
use clap::{Args, ValueEnum};
use qrcode_generator::{self, QrCodeEcc};
use std::fs;
use std::path::PathBuf;

const WPA_MAX_PASSWORD_LENGTH: usize = 63;

const QR_IMAGE_SIZE_PIXELS: usize = 300;

/// The level of QR code error correction. See `qrcode_generator::QrCodeEcc`
/// for details on what these mean.
///
/// We define our own enum so we can implement some traits for command line argument parsing.
#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
enum ErrorCorrection {
    Low,
    Medium,
    Quartile,
    High,
}

impl Default for ErrorCorrection {
    fn default() -> Self {
        Self::Medium
    }
}

impl ErrorCorrection {
    fn to_upstream(&self) -> QrCodeEcc {
        match self {
            ErrorCorrection::Low => QrCodeEcc::Low,
            ErrorCorrection::Medium => QrCodeEcc::Medium,
            ErrorCorrection::Quartile => QrCodeEcc::Quartile,
            ErrorCorrection::High => QrCodeEcc::High,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
enum ImageFormat {
    Png,
    Svg,
}

fn wifiqr_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        if c == '\\' || c == '"' || c == ';' || c == ':' || c == ',' {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

pub(crate) fn wifiqr_encode(ssid: &str, is_hidden: bool, password: &Secret) -> Result<Secret> {
    // TODO: Do this in-place.
    let data = format!(
        "WIFI:S:{};T:WPA;P:{};H:{};;",
        wifiqr_escape(&ssid),
        wifiqr_escape(std::str::from_utf8(unsafe { password.as_slice() })?),
        match is_hidden {
            false => "false",
            true => "true",
        }
    )
    .into_bytes();

    let mut s = Secret::with_len(data.len())?;
    unsafe {
        s.as_mut_slice().copy_from_slice(data.as_slice());
    }
    Ok(s)
}

#[derive(Args)]
pub(crate) struct WifiqrArgs {
    #[arg(short = 's', long)]
    /// The wireless network SSID (name).
    ssid: String,

    #[arg(long)]
    /// Set this if the network SSID is hidden / not broadcasted.
    hidden: bool,

    #[arg(long)]
    /// Instead of generating a new password, prompt for an existing password.
    prompt: bool,

    #[arg(short = 'l', long, default_value_t = WPA_MAX_PASSWORD_LENGTH)]
    /// The length of the password to generate.
    password_length: usize,

    #[command(flatten)]
    generate: GenerateArgs,

    #[arg(short = 'e', long)]
    /// The amount of error correction to include in the QR code.
    error_correction: ErrorCorrection,

    #[arg(short = 'o', long)]
    /// The path to write the output to. Format autodetected from file extension.
    output: PathBuf,

    #[arg(long)]
    /// Set this if you explicitly want to overwrite an existing output file.
    overwrite: bool,
}

pub(crate) fn wifiqr_command(args: WifiqrArgs) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();

    let password = if args.prompt {
        password_prompt("Password:", /*confirm=*/ true)?
    } else {
        let charsets = args.generate.to_charsets();
        let custom_exclude: Vec<char> = args
            .generate
            .custom_exclude
            .map_or(vec![], |x| x.chars().collect());
        pwgen::generate_password(args.password_length, &charsets, &custom_exclude)?
    };

    // Determine the image format first; if the extension is invalid, we want
    // to return an error before writing anything to disk.
    let format: ImageFormat = match args.output.extension().map(|ext| ext.to_str()).flatten() {
        Some("png") => ImageFormat::Png,
        Some("svg") => ImageFormat::Svg,
        None => bail!(
            "invalid output path '{}', file extension is not valid UTF-8",
            args.output.display()
        ),
        _ => bail!(
            "invalid output path '{}', expected PNG or SVG extension",
            args.output.display()
        ),
    };

    // TODO: Don't do this check, use File::open or File::create instead.
    // Check if the output already exists, and create its parent directory.
    if args.output.exists() {
        if !args.overwrite {
            bail!("refusing to overwrite '{}'", args.output.display());
        }
        if !args.output.is_file() {
            bail!(
                "found directory, expected file at '{}'",
                args.output.display()
            );
        }
    }
    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the QR code to the output path.
    let encoded = wifiqr_encode(&args.ssid, args.hidden, &password)?;
    match format {
        ImageFormat::Png => qrcode_generator::to_png_to_file(
            unsafe { encoded.as_slice() },
            args.error_correction.to_upstream(),
            QR_IMAGE_SIZE_PIXELS,
            args.output,
        )?,
        ImageFormat::Svg => qrcode_generator::to_svg_to_file::<&[u8], String, PathBuf>(
            unsafe { encoded.as_slice() },
            args.error_correction.to_upstream(),
            QR_IMAGE_SIZE_PIXELS,
            None,
            args.output,
        )?,
    };

    // Also print out the secret as plain text.
    output_secret(&password, InputEncoding::Auto, OutputMethod::Stdout)?;

    Ok(())
}
