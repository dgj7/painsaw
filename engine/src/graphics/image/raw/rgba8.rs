use crate::graphics::image::raw::ri::Pixel;
use crate::graphics::model::color::{convert_u8, Color};

///
/// opengl GL_RGBA (internal GL_RGBA8).
///
#[allow(dead_code)]// todo: remove this
pub(crate) struct PixelRgba8 {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl Pixel for PixelRgba8 {}

impl PixelRgba8 {
    #[allow(dead_code)]// todo: remove this
    pub(crate) fn from_bit_per_pixel(bit: bool, on_color: &Color, off_color: &Color) -> PixelRgba8 {
        if bit {
            let (r, g, b, a) = convert_u8(on_color);
            PixelRgba8 { r, g, b, a}
        } else {
            let (r, g, b, a) = convert_u8(off_color);
            PixelRgba8 { r, g, b, a}
        }
    }
}
