// Prism Screenshot Clipboard Helper
// Uses arboard to copy text and captured images directly to X11/Wayland system clipboards.

use arboard::{Clipboard, ImageData};
use std::borrow::Cow;

/// Copy a plain string to the system clipboard
pub fn copy_text(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text.to_string()).map_err(|e| e.to_string())?;
    Ok(())
}

/// Copy an image to the system clipboard
pub fn copy_image(img: &image::RgbaImage) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    let img_data = ImageData {
        width: img.width() as usize,
        height: img.height() as usize,
        bytes: Cow::Borrowed(img.as_raw()),
    };
    clipboard.set_image(img_data).map_err(|e| e.to_string())?;
    Ok(())
}
