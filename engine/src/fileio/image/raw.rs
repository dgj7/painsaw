//!
//! various "raw" image formats; these are the ones uploaded to the graphics chip.
//!

use crate::fileio::resources::memory::MemoryResource;
use crate::fileio::resources::Resource;
use crate::graphics::model::color::Color;

pub mod rgba8;
pub mod rgba8f;
pub mod rgba32ui;
pub mod rgba32i;

///
/// marker trait for any pixel type.
///
pub trait Pixel {}

///
/// raw opengl image data.
///
pub struct RawImage<P: Pixel> {
    pub width: u32,
    pub height: u32,
    pub data: Vec<P>,
}

impl<P: Pixel> RawImage<P> {
    ///
    /// Convert from an array of bytes, presumed to be encoded as one bit-per-pixel,
    /// into the given pixel format.
    ///
    #[allow(dead_code)]// todo: remove this
    pub fn one_bpp_to_raw_img<F>(width: u32, height: u32, resource: Box<dyn Resource>, on_color: &Color, off_color: &Color, func: F) -> RawImage<P>
    where
        F: Fn(bool, &Color, &Color) -> P,
    {
        /* intermediary data storage */
        let mut data = vec!();
        let bytes = match resource.bytes() {
            Ok(bytes) => bytes,
            Err(e) => panic!("[{}]: error loading bytes", e)
        };

        /* iterate over bits and expand */
        for byte in bytes {
            for i in 0..8 {
                let bit = ((byte >> i) & 1) != 0;
                let expanded = func(bit, on_color, off_color);
                data.push(expanded);
            }
        }

        /* done */
        RawImage {
            width,
            height,
            data,
        }
    }
}
