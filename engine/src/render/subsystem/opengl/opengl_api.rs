use windows::Win32::Graphics::OpenGL::{glBegin, glClear, glClearColor, glColor3f, glDisable, glEnable, glEnd, glFlush, glFrustum, glLineWidth, glLoadIdentity, glMatrixMode, glOrtho, glVertex2f, glVertex3f, glViewport, GL_LINES};
use crate::render::subsystem::opengl::opengl_errors::check_errors_gl;

pub(crate) fn gl_clear(mask: u32) {
    unsafe { glClear(mask); }
    check_errors_gl("glClear");
}

pub(crate) fn gl_clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glClearColor(red, green, blue, alpha); }
    check_errors_gl("glClearColor");
}

#[allow(unused)]
pub(crate) fn gl_frustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    unsafe { glFrustum(left, right, bottom, top, znear, zfar); }
    check_errors_gl(&format!("glFrustum({},{},{},{},{},{})", left, right, bottom, top, znear, zfar));
}

pub(crate) fn gl_begin_lines() {
    unsafe { glBegin(GL_LINES); }
}

pub(crate) fn gl_end() {
    unsafe { glEnd(); }
    check_errors_gl("glEnd");
}

#[allow(unused)]
pub(crate) fn gl_flush() {
    unsafe { glFlush(); }
    check_errors_gl("glFlush");
}

pub(crate) fn gl_viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { glViewport(x, y, width, height); }
    check_errors_gl("glViewport");
}

pub(crate) fn gl_matrix_mode(mode: u32) {
    unsafe { glMatrixMode(mode) }
    check_errors_gl("glMatrixMode");
}

pub(crate) fn gl_load_identity() {
    unsafe { glLoadIdentity() }
    check_errors_gl("glLoadIdentity");
}

pub(crate) fn gl_ortho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    unsafe { glOrtho(left, right, bottom, top, znear, zfar) }
    check_errors_gl("glOrtho");
}

pub(crate) fn gl_enable(cap: u32) {
    unsafe { glEnable(cap); }
    check_errors_gl("glEnable");
}

pub(crate) fn gl_disable(cap: u32) {
    unsafe { glDisable(cap); }
    check_errors_gl("glDisable");
}

pub(crate) fn gl_line_width(width_pixels: f32) {
    unsafe { glLineWidth(width_pixels); }
    check_errors_gl("glLineWidth");
}

pub(crate) fn gl_color_3f(red: f32, green: f32, blue: f32) {
    unsafe { glColor3f(red, green, blue); }
    check_errors_gl("glColor3f");
}

pub(crate) fn gl_vertex_2f(x: f32, y: f32) {
    unsafe { glVertex2f(x, y); }
}

pub(crate) fn gl_vertex_3f(x: f32, y: f32, z: f32) {
    unsafe { glVertex3f(x, y, z); }
}
