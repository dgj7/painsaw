//!
//! various "raw" image formats; these are the ones uploaded to the graphics chip.
//!

use crate::fileio::resources::Resource;
use crate::graphics::model::color::Color;

pub enum Pixel {
    // opengl GL_RGBA (internal GL_RGBA32F)
    PixelRgba8f {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },

    // opengl GL_RGBA (internal GL_RGBA8)
    PixelRgba8 {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    },

    // opengl GL_RGBA_INTEGER (internal GL_RGBA32I)
    PixelRgba32i {
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    },

    // opengl GL_RGBA_INTEGER (internal GL_RGBA32UI)
    PixelRgba32ui {
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    },
}

impl Pixel {
    pub fn bit_per_pixel(bit: bool, on_color: &Color, off_color: &Color) -> (u8, u8, u8, u8) {
        if bit {
            on_color.to_u8()
        } else {
            off_color.to_u8()
        }
    }
}

///
/// raw opengl image data.
///
pub struct RawImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl RawImage {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> RawImage {
        RawImage {
            width,
            height,
            data,
        }
    }

    ///
    /// Convert from an array of bytes, presumed to be encoded as one bit-per-pixel,
    /// into the given pixel format.
    ///
    pub fn one_bpp_to_raw_img<F>(width: u32, height: u32, resource: Box<dyn Resource>, on_color: &Color, off_color: &Color, func: F) -> RawImage
    where
        F: Fn(bool, &Color, &Color) -> (u8, u8, u8, u8),
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
                data.push(expanded.0);
                data.push(expanded.1);
                data.push(expanded.2);
                data.push(expanded.3);
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
