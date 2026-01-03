use crate::fileio::image::raw::Pixel;

///
/// opengl GL-RRGBA (internal GL_RGBA32F).
///
#[allow(dead_code)]// todo: remove this
pub(crate) struct PixelRgba8f {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
    pub(crate) a: f32,
}

impl Pixel for PixelRgba8f {}
