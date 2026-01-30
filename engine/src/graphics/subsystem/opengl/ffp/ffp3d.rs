use crate::config::{CVAR_FOV, DEFAULT_FOV};
use crate::graphics::geometry::primitive::prim3d::Primitive3D;
use crate::graphics::subsystem::opengl::ffp::api::{gl_begin_lines, gl_begin_points, gl_color_3f, gl_enable, gl_end, gl_line_width, gl_load_identity, gl_matrix_mode, gl_point_size, gl_pop_attrib, gl_pop_matrix, gl_push_attrib, gl_push_matrix, gl_rotate_f, gl_scale_f, gl_translate_f, gl_vertex_3f, glu_perspective};
use crate::window::context::RendererContext;
use num_traits::Float;
use windows::Win32::Graphics::OpenGL::{GL_ALL_ATTRIB_BITS, GL_DEPTH_TEST, GL_MODELVIEW, GL_PROJECTION};
use crate::graphics::geometry::orient::Orientation;

pub(crate) fn ffp_3d_setup<F: Float>(context: &RendererContext<F>) {
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
    let fov: f64 = context.config.get_cvar(CVAR_FOV, |x| x.parse().unwrap()).unwrap_or(DEFAULT_FOV);
    glu_perspective(fov, camera.aspect(), camera.near.to_f64().unwrap(), camera.far.to_f64().unwrap());

    /* storage/view: reset matrix; enable depth test; ready for 3d drawing */
    gl_matrix_mode(GL_MODELVIEW);
    gl_load_identity();
    gl_enable(GL_DEPTH_TEST);

    /* storage/view: adjust camera, before drawing */
    gl_rotate_f(-camera.orientation.pitch().to_f32().unwrap(), 1.0, 0.0, 0.0);
    gl_rotate_f(-camera.orientation.yaw().to_f32().unwrap(), 0.0, 1.0, 0.0);
    gl_translate_f(-position.x.to_f32().unwrap(), -position.y.to_f32().unwrap(), -position.z.to_f32().unwrap());
}

pub(crate) fn ffp_3d_teardown() {
    /* storage/view: reset matrix */
    //gl_matrix_mode(GL_MODELVIEW);
    //gl_load_identity();

    /* disable stuff we don't need anymore */
    //gl_disable(GL_DEPTH_TEST);


    gl_pop_attrib();
    gl_pop_matrix();
}

fn ffp_3d_translate<F: Float>(orientation: &Orientation<F>) {
    let position = orientation.position.column_major_position();
    let yaw = orientation.yaw().to_f32().unwrap();
    let pitch = orientation.pitch().to_f32().unwrap();
    gl_translate_f(position.x.to_f32().unwrap(), position.y.to_f32().unwrap(), position.z.to_f32().unwrap());
    gl_rotate_f(pitch, 1.0, 0.0, 0.0);
    gl_rotate_f(yaw, 0.0, 1.0, 0.0);
    gl_scale_f(orientation.x_scale.to_f32().unwrap(), orientation.y_scale.to_f32().unwrap(), orientation.z_scale.to_f32().unwrap());
}

pub(crate) fn ffp_3d_points<F: Float>(primitive: &Primitive3D<F>, point_size: F) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    ffp_3d_translate(&primitive.orientation);

    gl_color_3f(primitive.color.red, primitive.color.green, primitive.color.blue);
    gl_point_size(point_size.to_f32().unwrap());

    gl_begin_points();
    for vert in &primitive.vertices {
        gl_vertex_3f(vert.x.to_f32().unwrap(), vert.y.to_f32().unwrap(), vert.z.to_f32().unwrap());
    }
    gl_end();

    gl_pop_attrib();
    gl_pop_matrix();
}

pub(crate) fn ffp_3d_lines<F: Float>(primitive: &Primitive3D<F>, thickness: F) {
    gl_push_matrix();
    gl_push_attrib(GL_ALL_ATTRIB_BITS);

    ffp_3d_translate(&primitive.orientation);

    gl_color_3f(primitive.color.red, primitive.color.green, primitive.color.blue);
    gl_line_width(thickness.to_f32().unwrap());

    gl_begin_lines();
    for vert in &primitive.vertices {
        gl_vertex_3f(vert.x.to_f32().unwrap(), vert.y.to_f32().unwrap(), vert.z.to_f32().unwrap());
    }
    gl_end();

    gl_pop_attrib();
    gl_pop_matrix();
}
