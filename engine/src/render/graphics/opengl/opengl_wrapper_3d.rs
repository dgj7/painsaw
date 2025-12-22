use crate::math::draw_config::DrawingConfig;
use crate::math::threed::line_3d::Line3D;
use crate::render::graphics::opengl::opengl_api::{gl_begin_lines, gl_color_3f, gl_enable, gl_end, gl_line_width, gl_load_identity, gl_matrix_mode, gl_vertex_3f};
use crate::render::model::render_context::RendererContext;
use windows::Win32::Graphics::OpenGL::{GL_DEPTH_TEST, GL_MODELVIEW, GL_PROJECTION};

//https://stackoverflow.com/questions/4280185/opengl-drawing-a-2d-overlay-on-a-3d-scene-problem
pub fn prepare_3d(_context: &mut RendererContext) {
    gl_enable(GL_DEPTH_TEST);

    gl_matrix_mode(GL_PROJECTION);
    gl_load_identity();
    // todo: do camera; gluPerspective, glFrustum
    gl_matrix_mode(GL_MODELVIEW);
    gl_load_identity();
}

pub fn paint_3d_lines(lines: &[Line3D], config: &DrawingConfig) {
    gl_color_3f(config.color.red, config.color.green, config.color.blue);
    gl_line_width(config.line_thickness);

    gl_begin_lines();
    for line in lines {
        gl_vertex_3f(line.a.x, line.a.y, line.a.z);
        gl_vertex_3f(line.b.x, line.b.y, line.b.z);
    }
    gl_end();
}
