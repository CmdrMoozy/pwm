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

use crate::crypto::pwgen;
use crate::error::*;
use crate::output::{output_secret, InputEncoding, OutputMethod};
use crate::secret::Secret;
use flaggy::*;
use qrcode_generator::{self, QrCodeEcc};
use std::fs;
use std::path::PathBuf;

const WPA_MAX_PASSWORD_LENGTH: usize = 63;
const WPA_PASSWORD_CHARSETS: &'static [pwgen::CharacterSet] = &[
    pwgen::CharacterSet::Letters,
    pwgen::CharacterSet::Numbers,
    pwgen::CharacterSet::Symbols,
];

const QR_IMAGE_SIZE_PIXELS: usize = 300;

/// The level of QR code error correction. See `qrcode_generator::QrCodeEcc`
/// for details on what these mean.
///
/// We define our own enum so we can make it `FromStr`, so we can parse command
/// line arguments.
enum ErrorCorrection {
    Low,
    Medium,
    Quartile,
    High,
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

impl std::str::FromStr for ErrorCorrection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "Low" => ErrorCorrection::Low,
            "Medium" => ErrorCorrection::Medium,
            "Quartile" => ErrorCorrection::Quartile,
            "High" => ErrorCorrection::High,
            _ => {
                return Err(Error::InvalidArgument(format!(
                    "invalid error correction '{}'",
                    s
                )))
            }
        })
    }
}

enum ImageFormat {
    Png,
    Svg,
}

impl std::str::FromStr for ImageFormat {
    type Err = Error;

    fn from_str(extension: &str) -> Result<Self> {
        Ok(match extension {
            "png" => ImageFormat::Png,
            "svg" => ImageFormat::Svg,
            _ => {
                return Err(Error::InvalidArgument(format!(
                    "invalid file extension '{}'; only *.png and *.svg are supported",
                    extension
                )))
            }
        })
    }
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
    Ok(format!(
        "WIFI:S:{};T:WPA;P:{};H:{};;",
        wifiqr_escape(&ssid),
        wifiqr_escape(std::str::from_utf8(password)?),
        match is_hidden {
            false => "false",
            true => "true",
        }
    )
    .into_bytes())
}

#[command_callback]
fn wifiqr(
    ssid: String,
    is_hidden: bool,
    error_correction: ErrorCorrection,
    output: PathBuf,
    overwrite: bool,
) -> Result<()> {
    let _handle = crate::init_with_configuration().unwrap();
    let password = pwgen::generate_password(WPA_MAX_PASSWORD_LENGTH, WPA_PASSWORD_CHARSETS, &[])?;

    // Determine the image format first; if the extension is invalid, we want
    // to return an error before writing anything to disk.
    let format: ImageFormat = if let Some(extension) = output.extension() {
        if let Some(extension_str) = extension.to_str() {
            extension_str.parse()?
        } else {
            return Err(Error::InvalidArgument(format!(
                "invalid output path '{}', file extension is not valid UTF-8",
                output.display()
            )));
        }
    } else {
        return Err(Error::InvalidArgument(format!(
            "invalid output path '{}', expected *.png or *.svg extension",
            output.display()
        )));
    };

    // Check if the output already exists, and create its parent directory.
    if output.exists() {
        if !overwrite {
            return Err(Error::InvalidArgument(format!(
                "refusing to overwrite '{}'",
                output.display()
            )));
        }
        if !output.is_file() {
            return Err(Error::InvalidArgument(format!(
                "found directory, expected file at '{}'",
                output.display()
            )));
        }
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the QR code to the output path.
    let encoded = wifiqr_encode(&ssid, is_hidden, &password)?;
    match format {
        ImageFormat::Png => qrcode_generator::to_png_to_file(
            encoded,
            error_correction.to_upstream(),
            QR_IMAGE_SIZE_PIXELS,
            output,
        )?,
        ImageFormat::Svg => qrcode_generator::to_svg_to_file::<Secret, String, PathBuf>(
            encoded,
            error_correction.to_upstream(),
            QR_IMAGE_SIZE_PIXELS,
            None,
            output,
        )?,
    };

    // Also print out the secret as plain text.
    output_secret(&password, InputEncoding::Auto, OutputMethod::Stdout)?;

    Ok(())
}

pub fn build_wifiqr_command() -> Command<'static, Error> {
    Command::new(
        "wifiqr",
        "Generate a WiFi password, and render it as a QR code for sharing.",
        Specs::new(vec![
            Spec::required("ssid", "The wireless network SSID.", Some('s'), None),
            Spec::boolean(
                "is_hidden",
                "Set this if the network SSID is hidden / not broadcasted.",
                Some('h'),
            ),
            Spec::required(
                "error_correction",
                "The amount of error correction to include in the QR code.",
                Some('e'),
                Some("Medium"),
            ),
            Spec::required(
                "output",
                "The path to write the output to. Format autodetected from file extension.",
                Some('o'),
                None,
            ),
            Spec::boolean(
                "overwrite",
                "Set this if you explicitly want to overwrite an existing output file.",
                None,
            ),
        ])
        .unwrap(),
        Box::new(wifiqr),
    )
}
