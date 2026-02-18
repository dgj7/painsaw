use crate::graphics::camera::Camera;
use crate::graphics::subsystem::opengl::ffp::api::{gl_clear, gl_clear_color, gl_viewport};
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use windows::Win32::Graphics::OpenGL::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT};

pub(crate) mod api;
pub mod ffp2d;
pub mod ffp3d;

pub(crate) fn ffp_before_scene() {
    gl_clear_color(0.0, 0.0, 0.0, 1.0);
    gl_clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

pub(crate) fn ffp_resize(camera: &Camera) {
    /* set the viewport; this call doesn't need a specific matrix mode as it's an independent function */
    gl_viewport(0, 0, camera.projection.width as i32, camera.projection.height as i32);

    /* observe and report */
    log(LogLevel::Debug, &|| String::from(format!("resize(): w=[{}],h=[{}]", camera.projection.width, camera.projection.height)));
}
