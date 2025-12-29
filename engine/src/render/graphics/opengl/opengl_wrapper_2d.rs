use crate::geometry::config::draw_config::DrawingConfig;
use crate::geometry::line::l2d::Line2D;
use crate::render::graphics::opengl::opengl_api::{gl_begin_lines, gl_clear, gl_clear_color, gl_color_3f, gl_disable, gl_end, gl_line_width, gl_load_identity, gl_matrix_mode, gl_ortho, gl_vertex_2f};
use crate::render::model::color::Color;
use crate::render::model::render_context::RendererContext;
use windows::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_DEPTH_TEST, GL_MODELVIEW, GL_PROJECTION};

pub fn paint_background(color: Color) {
    gl_clear_color(color.red, color.green, color.blue, color.alpha);
    gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

pub fn prepare_2d(context: &mut RendererContext) {
    gl_disable(GL_DEPTH_TEST);
    let ccd = context.copy_client_dimensions();
    gl_matrix_mode(GL_PROJECTION);
    gl_load_identity();
    gl_ortho(0.0,
             ccd.width as f64,
             ccd.height as f64,
             0.0,
             -99999.0,
             99999.0
    );
    gl_matrix_mode(GL_MODELVIEW);
}

pub fn paint_2d_lines(lines: &[Line2D<f32>], config: &DrawingConfig) {
    gl_color_3f(config.color.red, config.color.green, config.color.blue);
    gl_line_width(config.line_thickness);
    
    gl_begin_lines();
    for line in lines {
        gl_vertex_2f(line.x.x, line.x.y);
        gl_vertex_2f(line.y.x, line.y.y);
    }
    gl_end();
}
