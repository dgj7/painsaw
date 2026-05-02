use crate::support::image::{Image, RawImage};
use std::io::{BufRead, Seek};

pub struct Tiff;

///
/// load tiff image files.
///
impl Image for Tiff {
    fn load_from_buf_read<R: BufRead + Seek>(_reader: R) -> std::io::Result<RawImage> {
        todo!()
    }
}
