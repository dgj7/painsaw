use crate::geometry::line::ls2d::Lines2D;
use crate::render::subsystem::opengl::opengl_api::{gl_begin_lines, gl_color_3f, gl_end, gl_line_width, gl_vertex_2f};
use num_traits::Float;

pub fn paint_2d_lines<F: Float>(lines: &Lines2D<F>) {
    gl_color_3f(lines.color.red, lines.color.green, lines.color.blue);
    gl_line_width(lines.thickness.to_f32().unwrap());
    
    gl_begin_lines();
    for line in lines.lines.iter() {
        gl_vertex_2f(line.x.x.to_f32().unwrap(), line.x.y.to_f32().unwrap());
        gl_vertex_2f(line.y.x.to_f32().unwrap(), line.y.y.to_f32().unwrap());
    }
    gl_end();
}
