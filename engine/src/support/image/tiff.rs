use std::path::Path;
use crate::support::image::RawImage;

///
/// load tiff image files.
///
pub fn load_tiff<P: AsRef<Path>>(_path: P) -> std::io::Result<RawImage> {
    todo!("implement tiff loading")
}
