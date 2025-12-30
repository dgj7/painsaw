use crate::wc::model::color::Color;

pub struct DrawingConfig {
    pub color: Color,
    pub line_thickness: f32,
}

impl DrawingConfig {
    pub fn new(color: Color, line_thickness: f32) -> DrawingConfig {
        DrawingConfig { color, line_thickness }
    }
}

impl Default for DrawingConfig {
    fn default() -> DrawingConfig {
        DrawingConfig::new(Color::BLACK, 1.0)
    }
}
