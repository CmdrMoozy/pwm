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

use crate::error::*;
use crate::output::{encode_for_display, InputEncoding, OutputHandler};
use crate::util::data::SecretSlice;
use clipboard::{self, ClipboardProvider};
use failure::format_err;
use lazy_static::lazy_static;
use log::info;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    static ref CLIPBOARD_TIMEOUT: Duration = Duration::new(45, 0);
}

fn set_contents_string<CP: ClipboardProvider>(cp: &mut CP, contents: String) -> Result<()> {
    match cp.set_contents(contents) {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(Error::Internal(format_err!(
                "Failed to set clipboard contents"
            )));
        }
    }
}

pub(crate) struct ClipboardOutputHandler;

impl OutputHandler for ClipboardOutputHandler {
    /// Set the contents of the OS's clipboard to the given data. If
    /// `force_binary` is true, or if the given data is determined to not be a
    /// valid UTF-8-encoded string, then the clipboard will be populated with a
    /// base-64 encoded version of the data.
    fn handle(&self, secret: &SecretSlice, encoding: InputEncoding) -> Result<()> {
        let mut cp: clipboard::x11_clipboard::X11ClipboardContext<
            clipboard::x11_clipboard::Clipboard,
        > = match clipboard::x11_clipboard::X11ClipboardContext::new() {
            Ok(cp) => cp,
            Err(_) => {
                return Err(Error::Internal(format_err!(
                    "Failed to get clipboard context"
                )));
            }
        };

        set_contents_string(
            &mut cp,
            String::from_utf8(encode_for_display(
                secret, encoding, /*supports_binary=*/ false,
            ))
            .unwrap(),
        )?;
        info!(
            "Copied stored password or key to clipboard. Will clear in {} seconds.",
            CLIPBOARD_TIMEOUT.as_secs()
        );
        sleep(*CLIPBOARD_TIMEOUT);
        set_contents_string(&mut cp, "".to_owned())?;

        Ok(())
    }
}