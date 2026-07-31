use crate::graphics::color::Color;
use crate::support::text::{TextConfig, Typeface};

pub mod coords;
pub mod fps;
pub mod screen;

pub(crate) static TC: TextConfig = TextConfig {
    foreground: Color::RED,
    background: Color::TRANSPARENT,
    typeface: Typeface::Generic,
};

pub(crate) static HEIGHT: f32 = 13.7;
pub(crate) static X_POS: f32 = 10.0;
