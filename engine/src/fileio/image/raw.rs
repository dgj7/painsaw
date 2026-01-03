//!
//! various "raw" image formats; these are the ones uploaded to the graphics chip.
//!

use crate::graphics::model::color::Color;

pub(crate) mod rgba8;
pub(crate) mod rgba8f;
pub(crate) mod rgba32ui;
pub(crate) mod rgba32i;

///
/// marker trait for any pixel type.
///
pub(crate) trait Pixel {}

///
/// raw opengl image data.
///
#[allow(dead_code)]// todo: remove this
pub(crate) struct RawImage<P: Pixel> {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<P>,
}

impl<P: Pixel> RawImage<P> {
    ///
    /// Convert from an array of bytes, presumed to be encoded as one bit-per-pixel,
    /// into the given pixel format.
    ///
    #[allow(dead_code)]// todo: remove this
    pub fn one_bpp_to_raw_img<F>(width: u32, height: u32, bytes: Vec<u8>, on_color: &Color, off_color: &Color, func: F) -> RawImage<P>
    where
        F: Fn(bool, &Color, &Color) -> P,
    {
        /* intermediary data storage */
        let mut data = vec!();

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
