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

#[cfg(feature = "clipboard")]
mod clipboard;
mod stdout;

use crate::error::*;
use crate::secret::{encode, Secret};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum InputEncoding {
    /// Simply assume the input is binary data, and cannot be interpreted as
    /// UTF-8.
    Binary,

    /// Automatically detect the input's encoding. Try to interpret it as UTF-8,
    /// and if this fails, assume it is arbitrary binary data.
    Auto,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OutputMethod {
    /// Write the output to stdout.
    Stdout,

    /// Store the output in the system's clipboard.
    #[cfg(feature = "clipboard")]
    Clipboard,
}

lazy_static! {
    static ref OUTPUT_METHOD_STRINGS: HashMap<OutputMethod, &'static str> = {
        let mut m = HashMap::new();
        m.insert(OutputMethod::Stdout, "Stdout");
        #[cfg(feature = "clipboard")]
        m.insert(OutputMethod::Clipboard, "Clipboard");
        m
    };
    static ref STRING_OUTPUT_METHODS: HashMap<String, OutputMethod> = {
        OUTPUT_METHOD_STRINGS
            .iter()
            .map(|pair| (pair.1.to_uppercase(), *pair.0))
            .collect()
    };
}

impl fmt::Display for OutputMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", OUTPUT_METHOD_STRINGS.get(self).map_or("", |s| *s))
    }
}

impl FromStr for OutputMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.to_uppercase();
        Ok(match STRING_OUTPUT_METHODS.get(&s) {
            None => {
                return Err(Error::InvalidArgument(format!(
                    "invalid output method '{}'",
                    s
                )))
            }
            Some(om) => *om,
        })
    }
}

/// This function takes the given `secret`, interprets it according to
/// `encoding`, possibly re-encodes the `secret` as base-64 according to
/// both `encoding` and `supports_binary`, and then returns the result as a
/// vector of bytes.
pub fn encode_for_display(
    secret: &Secret,
    encoding: InputEncoding,
    supports_binary: bool,
) -> Secret {
    let as_utf8 = ::std::str::from_utf8(secret);
    let is_binary = match encoding {
        InputEncoding::Binary => true,
        InputEncoding::Auto => as_utf8.is_err(),
    };

    if !is_binary {
        secret.to_owned()
    } else if supports_binary {
        secret.to_owned()
    } else {
        encode(secret).into_bytes()
    }
}

trait OutputHandler {
    fn handle(&self, secret: &Secret, encoding: InputEncoding) -> Result<()>;
}

fn get_handler(method: OutputMethod) -> Box<dyn OutputHandler> {
    match method {
        OutputMethod::Stdout => Box::new(stdout::StdoutOutputHandler {}),
        #[cfg(feature = "clipboard")]
        OutputMethod::Clipboard => Box::new(clipboard::ClipboardOutputHandler {}),
    }
}

pub fn output_secret(secret: &Secret, encoding: InputEncoding, method: OutputMethod) -> Result<()> {
    get_handler(method).handle(secret, encoding)
}
