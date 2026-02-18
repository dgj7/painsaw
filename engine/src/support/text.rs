use crate::graphics::color::Color;
use crate::support::text::generic::create_generic;
use crate::graphics::image::RawImage;

pub mod generic;

#[derive(Clone)]
pub enum Typeface {
    Generic,
}

#[derive(Clone)]
pub struct TextConfig {
    pub typeface: Typeface,
    pub foreground: Color,
    pub background: Color,
}

pub fn text_2d_image<P>(config: TextConfig, provider: P) -> RawImage
where
    P: Fn() -> String,
{
    match config.typeface {
        Typeface::Generic => create_generic(&config, provider()),
    }
}

impl TextConfig {}

impl Default for TextConfig {
    fn default() -> Self {
        TextConfig {
            typeface: Typeface::Generic,
            foreground: Color::WHITE,
            background: Color::TRANSPARENT,
        }
    }
}
