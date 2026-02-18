use crate::geometry::orient::Orientation;
use crate::geometry::primitive::prim3d::Primitive3D;
use crate::graphics::subsystem::opengl::ffp::api::{gl_begin_lines, gl_begin_points, gl_begin_quads, gl_color_4f, gl_disable, gl_enable, gl_end, gl_line_width, gl_load_identity, gl_matrix_mode, gl_point_size, gl_polygon_mode, gl_pop_attrib, gl_pop_matrix, gl_push_attrib, gl_push_matrix, gl_rotate_f, gl_scale_f, gl_translate_f, gl_vertex_3f, glu_perspective};
use crate::window::context::RendererContext;
use windows::Win32::Graphics::OpenGL::{
    GL_ALL_ATTRIB_BITS, GL_DEPTH_TEST, GL_FRONT_AND_BACK, GL_LINE, GL_MODELVIEW, GL_PROJECTION,
};

pub(crate) fn ffp_3d_setup(context: &RendererContext) {
    /* save prior state before 3d rendering */
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    /* gather camera data */
    let camera = &context.camera;
    let position = camera.orientation.position.column_major_position();

    /* projection: reset matrix */
    gl_matrix_mode(GL_PROJECTION);
    gl_load_identity();

    /* adjust perspective (removes ortho) */
    glu_perspective(camera.projection.fov as f64, camera.projection.to_aspect() as f64, camera.projection.near as f64, camera.projection.far as f64);

    /* storage/view: reset matrix; enable depth test; ready for 3d drawing */
    gl_matrix_mode(GL_MODELVIEW);
    gl_load_identity();
    gl_enable(GL_DEPTH_TEST);

    /* storage/view: adjust camera, before drawing */
    gl_rotate_f(-camera.orientation.pitch(), 1.0, 0.0, 0.0);
    gl_rotate_f(-camera.orientation.yaw(), 0.0, 1.0, 0.0);
    gl_translate_f(-position.x, -position.y, -position.z);
}

pub(crate) fn ffp_3d_teardown() {
    gl_disable(GL_DEPTH_TEST);
    
    gl_pop_attrib();
    gl_pop_matrix();
}

fn ffp_3d_translate(orientation: &Orientation) {
    let position = orientation.position.column_major_position();
    let yaw = orientation.yaw();
    let pitch = orientation.pitch();
    gl_translate_f(position.x, position.y, position.z);
    gl_rotate_f(pitch, 1.0, 0.0, 0.0);
    gl_rotate_f(yaw, 0.0, 1.0, 0.0);
    gl_scale_f(orientation.x_scale, orientation.y_scale, orientation.z_scale);
}

pub(crate) fn ffp_3d_points(primitive: &Primitive3D, point_size: f32) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    ffp_3d_translate(&primitive.orientation);

    gl_color_4f(primitive.color.red, primitive.color.green, primitive.color.blue, primitive.color.alpha);
    gl_point_size(point_size);

    gl_begin_points();
    for vert in &primitive.vertices {
        gl_vertex_3f(vert.x, vert.y, vert.z);
    }
    gl_end();

    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_3d_lines(primitive: &Primitive3D, thickness: f32) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    ffp_3d_translate(&primitive.orientation);

    gl_color_4f(primitive.color.red, primitive.color.green, primitive.color.blue, primitive.color.alpha);
    gl_line_width(thickness);

    gl_begin_lines();
    for vert in &primitive.vertices {
        gl_vertex_3f(vert.x, vert.y, vert.z);
    }
    gl_end();

    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_3d_quads(primitive: &Primitive3D) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    ffp_3d_translate(&primitive.orientation);
    gl_polygon_mode(GL_FRONT_AND_BACK, GL_LINE);

    gl_color_4f(primitive.color.red, primitive.color.green, primitive.color.blue, primitive.color.alpha);
    //gl_line_width(thickness.to_f32().unwrap());

    gl_begin_quads();
    for vert in &primitive.vertices {
        gl_vertex_3f(vert.x, vert.y, vert.z);
    }
    gl_end();

    gl_pop_attrib();
    gl_pop_matrix();
}
