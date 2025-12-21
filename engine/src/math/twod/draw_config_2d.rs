use crate::render::model::color::Color;

pub struct DrawingConfig2D {
    pub color: Color,
    pub line_thickness: f32,
}

impl DrawingConfig2D {
    pub fn new(color: Color, line_thickness: f32) -> DrawingConfig2D {
        DrawingConfig2D { color, line_thickness }
    }
}

impl Default for DrawingConfig2D {
    fn default() -> DrawingConfig2D {
        DrawingConfig2D::new(Color::BLACK, 1.0)
    }
}
