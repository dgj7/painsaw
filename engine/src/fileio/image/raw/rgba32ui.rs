use crate::fileio::image::raw::Pixel;

///
/// opengl GL_RGBA_INTEGER (internal GL_RGBA32UI).
///
#[allow(dead_code)]// todo: remove this
pub(crate) struct PixelRgba32ui {
    pub(crate) r: u32,
    pub(crate) g: u32,
    pub(crate) b: u32,
    pub(crate) a: u32,
}

impl Pixel for PixelRgba32ui {}
