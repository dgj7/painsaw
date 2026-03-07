use std::path::Path;
use crate::support::image::RawImage;

///
/// load targa image files.
///
pub fn load_targa<P: AsRef<Path>>(_path: P) -> std::io::Result<RawImage> {
    todo!("implement targa loading")
}
