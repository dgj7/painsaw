use crate::fileio::image::raw::{Pixel, RawImage};

pub struct Texture2D {
    id: u32,
    width: u32,
    height: u32,
}

impl Texture2D {
    pub fn new<P: Pixel>(image: &RawImage<P>, texture_id: u32) -> Texture2D {
        Texture2D {
            id: texture_id,
            width: image.width,
            height: image.height,
        }
    }
}
