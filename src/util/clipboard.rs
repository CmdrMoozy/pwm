// pwm - A simple password manager for Linux.
// Copyright (C) 2015  Axel Rasmussen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use clipboard;
use error::*;
use std::thread::sleep;
use std::time::Duration;
use util::data::SensitiveData;

lazy_static! {
    static ref CLIPBOARD_TIMEOUT: Duration = Duration::new(45, 0);
}

fn set_contents_string(ctx: &mut clipboard::ClipboardContext, contents: String) -> Result<()> {
    match ctx.set_contents(contents) {
        Ok(_) => Ok(()),
        Err(_) => bail!("Failed to set clipboard contents"),
    }
}

/// Set the contents of the OS's clipboard to the given data. If `force_binary`
/// is true, or if the given data is determined to not be a valid UTF-8-encoded
/// string, then the clipboard will be populated with a Base64 encoded version
/// of the data.
pub fn set_contents(data: SensitiveData, force_binary: bool) -> Result<()> {
    let mut ctx = match clipboard::ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(_) => bail!("Failed to get clipboard context"),
    };

    let as_utf8 = data.to_utf8();
    let binary = force_binary || as_utf8.is_err();
    let contents: String = if !binary {
        try!(as_utf8)
    } else {
        data.to_string()
    };

    try!(set_contents_string(&mut ctx, contents));

    info!("Copied stored password or key to clipboard. Will clear in {} seconds.",
          CLIPBOARD_TIMEOUT.as_secs());
    sleep(*CLIPBOARD_TIMEOUT);
    try!(set_contents_string(&mut ctx, "".to_owned()));

    Ok(())
}
