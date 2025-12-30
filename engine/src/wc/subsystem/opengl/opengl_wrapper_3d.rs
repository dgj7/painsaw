use crate::geometry::line::ls3d::Lines3D;
use crate::wc::subsystem::opengl::opengl_api::{gl_begin_lines, gl_color_3f, gl_end, gl_line_width, gl_vertex_3f};
use num_traits::Float;
//https://stackoverflow.com/questions/4280185/opengl-drawing-a-2d-overlay-on-a-3d-scene-problem

pub fn paint_3d_lines<F: Float>(lines: &Lines3D<F>) {
    gl_color_3f(lines.color.red, lines.color.green, lines.color.blue);
    gl_line_width(lines.thickness.to_f32().unwrap());

    gl_begin_lines();
    for line in &lines.lines {
        gl_vertex_3f(line.a.x.to_f32().unwrap(), line.a.y.to_f32().unwrap(), line.a.z.to_f32().unwrap());
        gl_vertex_3f(line.b.x.to_f32().unwrap(), line.b.y.to_f32().unwrap(), line.b.z.to_f32().unwrap());
    }
    gl_end();
}
