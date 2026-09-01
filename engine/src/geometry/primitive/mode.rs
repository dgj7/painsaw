use glcore::{GL_FILL, GL_LINE};
use windows::Win32::Graphics::OpenGL::GL_POINT;

pub enum PolygonMode {
    Line,
    Point,
    Fill,
}

impl PolygonMode {
    pub fn to_u32(&self) -> u32 {
        match self {
            PolygonMode::Fill => GL_FILL,
            PolygonMode::Line => GL_LINE,
            PolygonMode::Point => GL_POINT,
        }
    }
}
