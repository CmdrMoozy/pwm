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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Base64(#[from] ::data_encoding::DecodeError),
    #[error(transparent)]
    Bdrck(#[from] ::bdrck::error::Error),
    /// An error encountered in deciphering command-line flag values.
    #[error(transparent)]
    CliFlags(#[from] ::flaggy::ValueError),
    #[error(transparent)]
    Git(#[from] ::git2::Error),
    /// An internal unrecoverable error, usually due to some underlying library.
    #[error("internal error: {0}")]
    Internal(String),
    /// Errors akin to EINVAL.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error(transparent)]
    Io(#[from] ::std::io::Error),
    #[error(transparent)]
    Json(#[from] ::serde_json::Error),
    /// An error encountered when decoding a serialized message.
    #[error(transparent)]
    MsgDecode(#[from] ::rmp_serde::decode::Error),
    /// An error encountered when encoding a struct to a serialized message.
    #[error(transparent)]
    MsgEncode(#[from] ::rmp_serde::encode::Error),
    /// Errors akin to ENOENT.
    #[error("not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    ParseBool(#[from] ::std::str::ParseBoolError),
    #[error(transparent)]
    ParseInt(#[from] ::std::num::ParseIntError),
    /// An error encountered while interacting with a PIV device.
    #[cfg(feature = "piv")]
    #[error(transparent)]
    PIV(#[from] ::yubirs::error::Error),
    /// An error occurred while generating a QR code.
    #[cfg(feature = "wifiqr")]
    #[error(transparent)]
    QRCode(#[from] ::qrcode_generator::QRCodeError),
    /// An awkward hack; this error exists to use String's FromStr impl, but
    /// this operation won't actually ever fail.
    #[error(transparent)]
    StringParse(#[from] ::std::string::ParseError),
    #[error("unknown error: {0}")]
    Unknown(String),
    #[error(transparent)]
    Utf8(#[from] ::std::string::FromUtf8Error),
    #[error(transparent)]
    Utf8Slice(#[from] ::std::str::Utf8Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;
