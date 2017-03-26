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

    try!(set_contents_string(&mut ctx, data.display(force_binary, true).unwrap()));

    info!("Copied stored password or key to clipboard. Will clear in {} seconds.",
          CLIPBOARD_TIMEOUT.as_secs());
    sleep(*CLIPBOARD_TIMEOUT);
    try!(set_contents_string(&mut ctx, "".to_owned()));

    Ok(())
}
