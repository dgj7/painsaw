use crate::math::twod::line_2d::Line2D;
use crate::render::graphics::opengl::opengl_api::{gl_begin_lines, gl_clear, gl_clear_color, gl_color_3f, gl_end, gl_flush, gl_line_width, gl_vertex_2f, gl_viewport};
use crate::render::model::color::Color;
use windows::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT};
use crate::math::twod::draw_config_2d::DrawingConfig2D;


pub fn adjust_viewport(width: i32, height: i32) {
    gl_viewport(0, 0, width, height);
}

pub fn paint_background(color: Color) {
    gl_clear_color(color.red, color.green, color.blue, color.alpha);
    gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

pub fn paint_lines(lines: &[Line2D], config: &DrawingConfig2D) {
    gl_color_3f(config.color.red, config.color.green, config.color.blue);
    gl_line_width(config.line_thickness);
    
    gl_begin_lines();
    for line in lines {
        gl_vertex_2f(line.x.x, line.x.y);
        gl_vertex_2f(line.y.x, line.y.y);
    }
    gl_end();
    
    gl_flush();
}
