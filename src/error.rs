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

use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Base64(#[cause] ::data_encoding::DecodeError),
    #[fail(display = "{}", _0)]
    Bdrck(#[cause] ::bdrck::error::Error),
    /// An error encountered in deciphering command-line flag values.
    #[fail(display = "{}", _0)]
    CliFlags(::failure::Error),
    #[fail(display = "{}", _0)]
    Git(#[cause] ::git2::Error),
    /// An internal unrecoverable error, usually due to some underlying library.
    #[fail(display = "{}", _0)]
    Internal(::failure::Error),
    /// Errors akin to EINVAL.
    #[fail(display = "{}", _0)]
    InvalidArgument(::failure::Error),
    #[fail(display = "{}", _0)]
    Io(#[cause] ::std::io::Error),
    #[fail(display = "{}", _0)]
    Json(::serde_json::Error),
    /// An error encountered when decoding a serialized message.
    #[fail(display = "{}", _0)]
    MsgDecode(#[cause] ::rmp_serde::decode::Error),
    /// An error encountered when encoding a struct to a serialized message.
    #[fail(display = "{}", _0)]
    MsgEncode(#[cause] ::rmp_serde::encode::Error),
    /// Errors akin to ENOENT.
    #[fail(display = "{}", _0)]
    NotFound(::failure::Error),
    #[fail(display = "{}", _0)]
    ParseBool(#[cause] ::std::str::ParseBoolError),
    #[fail(display = "{}", _0)]
    ParseInt(#[cause] ::std::num::ParseIntError),
    /// An error encountered while interacting with a PIV device.
    #[cfg(feature = "piv")]
    #[fail(display = "{}", _0)]
    PIV(#[cause] ::yubirs::error::Error),
    /// An awkward hack; this error exists to use String's FromStr impl, but
    /// this operation won't actually ever fail.
    #[fail(display = "{}", _0)]
    StringParse(#[cause] ::std::string::ParseError),
    #[fail(display = "{}", _0)]
    Unknown(::failure::Error),
    #[fail(display = "{}", _0)]
    Utf8(::std::string::FromUtf8Error),
}

impl From<::bdrck::error::Error> for Error {
    fn from(e: ::bdrck::error::Error) -> Self {
        Error::Bdrck(e)
    }
}

impl From<::flaggy::ValueError> for Error {
    fn from(e: ::flaggy::ValueError) -> Self {
        Error::CliFlags(failure::format_err!("{}", e))
    }
}

impl From<::git2::Error> for Error {
    fn from(e: ::git2::Error) -> Self {
        Error::Git(e)
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<::serde_json::Error> for Error {
    fn from(e: ::serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<::rmp_serde::decode::Error> for Error {
    fn from(e: ::rmp_serde::decode::Error) -> Self {
        Error::MsgDecode(e)
    }
}

impl From<::rmp_serde::encode::Error> for Error {
    fn from(e: ::rmp_serde::encode::Error) -> Self {
        Error::MsgEncode(e)
    }
}

impl From<::std::str::ParseBoolError> for Error {
    fn from(e: ::std::str::ParseBoolError) -> Self {
        Error::ParseBool(e)
    }
}

impl From<::std::num::ParseIntError> for Error {
    fn from(e: ::std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

#[cfg(feature = "piv")]
impl From<::yubirs::error::Error> for Error {
    fn from(e: ::yubirs::error::Error) -> Self {
        Error::PIV(e)
    }
}

impl From<::std::string::ParseError> for Error {
    fn from(e: ::std::string::ParseError) -> Self {
        Error::StringParse(e)
    }
}

impl From<::failure::Error> for Error {
    fn from(e: ::failure::Error) -> Self {
        Error::Unknown(e)
    }
}

impl From<::std::string::FromUtf8Error> for Error {
    fn from(e: ::std::string::FromUtf8Error) -> Self {
        Error::Utf8(e)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
