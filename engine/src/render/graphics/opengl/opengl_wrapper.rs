use windows::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT};
use crate::render::graphics::opengl::opengl_api::{gl_clear, gl_clear_color};
use crate::render::model::color::Color;

pub fn paint_background(color: Color) {
    gl_clear_color(color.red, color.green, color.blue, color.alpha);
    gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}
