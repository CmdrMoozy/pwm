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
use std::io::{self, Write};

pub(crate) struct StdoutOutputHandler;

impl OutputHandler for StdoutOutputHandler {
    fn handle(&self, secret: &SecretSlice, encoding: InputEncoding) -> Result<()> {
        let tty = bdrck::cli::isatty(bdrck::cli::Stream::Stdout);
        let display = encode_for_display(secret, encoding, /*supports_binary=*/ !tty);
        let mut stdout = io::stdout();
        stdout.write_all(&display)?;
        if tty {
            stdout.write_all(b"\n")?;
        }
        stdout.flush()?;
        Ok(())
    }
}