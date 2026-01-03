use crate::fileio::image::raw::ri::Pixel;

///
/// opengl GL_RGBA_INTEGER (internal GL_RGBA32I).
///
#[allow(dead_code)]// todo: remove this
pub(crate) struct PixelRgba32i {
    pub(crate) r: i32,
    pub(crate) g: i32,
    pub(crate) b: i32,
    pub(crate) a: i32,
}

impl Pixel for PixelRgba32i {}
