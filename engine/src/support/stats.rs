use crate::graphics::color::Color;
use crate::support::text::{TextConfig, Typeface};

pub mod coords;
pub mod fps;

pub(crate) static TC: TextConfig = TextConfig {
    foreground: Color::RED,
    background: Color::TRANSPARENT,
    typeface: Typeface::Generic,
};
