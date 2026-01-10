use crate::fileio::image::tex::t2d::Texture2D;
use crate::geometry::vector::p2d::Point2D;
use crate::graphics::model::color::Color;
use crate::text::generic::create_generic;
use num_traits::Float;

pub mod generic;

enum Typeface {
    Generic,
}

pub struct TextConfig<F: Float> {
    typeface: Typeface,
    top_left: Point2D<F>,
    scale: F,
    foreground: Color,
    background: Color,
}

pub fn text_2d<P, F: Float>(config: TextConfig<F>, provider: P) -> Texture2D<F>
where
    P: Fn() -> String,
{
    let image = match config.typeface {
        Typeface::Generic => create_generic(&config, provider()),
    };
    Texture2D::new(image, config.top_left, config.scale)
}

impl<F: Float> TextConfig<F> {}

impl<F: Float> Default for TextConfig<F> {
    fn default() -> Self {
        TextConfig {
            typeface: Typeface::Generic,
            top_left: Point2D::new(F::zero(), F::zero()),
            scale: F::one(),
            foreground: Color::WHITE,
            background: Color::BLACK,
        }
    }
}
