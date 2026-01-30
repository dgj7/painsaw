use num_traits::Float;
use windows::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT};
use crate::graphics::camera::Camera;
use crate::graphics::subsystem::opengl::ffp::api::{gl_clear, gl_clear_color, gl_viewport};
use crate::logger::log;
use crate::logger::log_level::LogLevel;

pub mod ffp2d;
pub(crate) mod api;

pub(crate) fn ffp_before_scene() {
    gl_clear_color(0.0, 0.0, 0.0, 1.0);
    gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

pub(crate) fn ffp_resize<F: Float>(camera: &Camera<F>) {
    /* set the viewport; this call doesn't need a specific matrix mode as it's an independent function */
    gl_viewport(0, 0, camera.width.to_i32().unwrap(), camera.height.to_i32().unwrap());

    /* observe and report */
    log(LogLevel::Debug, &|| String::from(format!("resize(): w=[{}],h=[{}]", camera.width.to_f32().unwrap(), camera.height.to_f32().unwrap())));
}
