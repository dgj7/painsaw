use crate::graphics::subsystem::opengl::errors::check_errors_gl;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use std::ffi::{c_char, CStr};
use windows::Win32::Graphics::OpenGL::{glBegin, glBindTexture, glBlendFunc, glClear, glClearColor, glColor4f, glDisable, glEnable, glEnd, glFrustum, glGenTextures, glGetString, glLineWidth, glLoadIdentity, glMatrixMode, glOrtho, glPointSize, glPolygonMode, glPopAttrib, glPopMatrix, glPushAttrib, glPushMatrix, glRotatef, glScalef, glTexCoord2f, glTexEnvf, glTexImage2D, glTexParameteri, glTexSubImage2D, glTranslatef, glVertex2f, glVertex3f, glViewport, gluPerspective, GL_LINES, GL_POINTS, GL_QUADS};

pub(crate) fn gl_clear(mask: u32) {
    unsafe { glClear(mask); }
    check_errors_gl("glClear");
}

pub(crate) fn gl_clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glClearColor(red, green, blue, alpha); }
    check_errors_gl("glClearColor");
}

#[allow(unused)] // todo: remove this
pub(crate) fn gl_frustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    unsafe { glFrustum(left, right, bottom, top, znear, zfar); }
    check_errors_gl(&format!("glFrustum({},{},{},{},{},{})", left, right, bottom, top, znear, zfar));
}

pub(crate) fn gl_begin(mode: u32) {
    unsafe { glBegin(mode) }
}

// todo: remove and replace with gl_begin
pub(crate) fn gl_begin_lines() {
    unsafe { glBegin(GL_LINES); }
}

// todo: remove and replace with gl_begin
pub(crate) fn gl_begin_points() {
    unsafe { glBegin(GL_POINTS); }
}

// todo: remove and replace with gl_begin
pub(crate) fn gl_begin_quads() {
    unsafe { glBegin(GL_QUADS); }
}

pub(crate) fn gl_end() {
    unsafe { glEnd(); }
    check_errors_gl("glEnd");
}

pub(crate) fn gl_viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { glViewport(x, y, width, height); }
    check_errors_gl("glViewport");
}

pub(crate) fn gl_matrix_mode(mode: u32) {
    unsafe { glMatrixMode(mode) }
    check_errors_gl("glMatrixMode");
}

#[allow(unused)] // todo: remove this
pub(crate) fn gl_push_matrix() {
    unsafe { glPushMatrix(); }
    check_errors_gl("glPushMatrix");
}

#[allow(unused)] // todo: remove this
pub(crate) fn gl_pop_matrix() {
    unsafe { glPopMatrix(); }
    check_errors_gl("glPopMatrix");
}

pub(crate) fn gl_load_identity() {
    unsafe { glLoadIdentity() }
    check_errors_gl("glLoadIdentity");
}

pub(crate) fn gl_ortho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    unsafe { glOrtho(left, right, bottom, top, znear, zfar) }
    check_errors_gl("glOrtho");
}

pub(crate) fn gl_get_string(name: u32) -> Option<String> {
    let result = unsafe { glGetString(name) };
    check_errors_gl("glGetString");

    if result.is_null() {
        return None;
    }

    let c_str = unsafe { CStr::from_ptr(result as *const c_char) };
    match c_str.to_str() {
        Ok(s) => Some(s.to_string()),
        Err(_e) => {
            log(LogLevel::Error, &|| String::from("glGetString returned invalid string"));
            None
        }
    }
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

pub(crate) fn gl_point_size(width_pixels: f32) {
    unsafe { glPointSize(width_pixels); }
    check_errors_gl("glPointSize");
}

pub(crate) fn gl_color_4f(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glColor4f(red, green, blue, alpha); }
    check_errors_gl("glColor4f");
}

pub(crate) fn gl_vertex_2f(x: f32, y: f32) {
    unsafe { glVertex2f(x, y); }
    //check_errors_gl("glVertex2f");
}

pub(crate) fn gl_vertex_3f(x: f32, y: f32, z: f32) {
    unsafe { glVertex3f(x, y, z); }
    //check_errors_gl("glVertex3f");
}

pub(crate) fn gl_gen_textures(n: i32, textures: *mut u32) {
    unsafe { glGenTextures(n, textures) }
    check_errors_gl("glGenTextures");
}

pub(crate) fn gl_bind_texture(target: u32, texture: u32) {
    unsafe { glBindTexture(target, texture); }
    check_errors_gl("glBindTexture");
}

pub(crate) fn gl_tex_parameter_i(target: u32, pname: u32, param2: i32) {
    unsafe { glTexParameteri(target, pname, param2) }
    check_errors_gl("glTexParameteri");
}

pub(crate) fn gl_tex_image_2d(target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, r#type: u32, pixels: *const core::ffi::c_void) {
    unsafe { glTexImage2D(target, level, internalformat, width, height, border, format, r#type, pixels) }
    check_errors_gl("glTexImage2D");
}

pub(crate) fn gl_tex_sub_image_2d(target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *const core::ffi::c_void) {
    unsafe { glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, r#type, pixels) }
    check_errors_gl("glTexSubImage2D");
}

pub(crate) fn gl_tex_coord_2f(s: f32, t: f32) {
    unsafe { glTexCoord2f(s, t) }
    //check_errors_gl("glTexCoord2f");
}

#[allow(unused)] // todo: remove this
pub(crate) fn gl_tex_env_f(target: u32, pname: u32, param2: f32) {
    unsafe { glTexEnvf(target, pname, param2) }
    check_errors_gl("glTexEnvf");
}

#[allow(unused)] // todo: remove this
pub(crate) fn gl_push_attrib(mask: u32) {
    unsafe { glPushAttrib(mask) }
    check_errors_gl("glPushAttrib");
}

#[allow(unused)] // todo: remove this
pub(crate) fn gl_pop_attrib() {
    unsafe { glPopAttrib() }
    check_errors_gl("glPopAttrib");
}

pub(crate) fn gl_blend_func(sfactor: u32, dfactor: u32) {
    unsafe { glBlendFunc(sfactor, dfactor) }
    check_errors_gl("glBlendFunc");
}

pub(crate) fn glu_perspective(fovy: f64, aspect: f64, znear: f64, zfar: f64) {
    unsafe { gluPerspective(fovy, aspect, znear, zfar) }
    check_errors_gl("gluPerspective");
}

#[allow(unused)] // todo: remove this
pub(crate) fn gl_translate_f(x: f32, y: f32, z: f32) {
    unsafe { glTranslatef(x, y, z) }
    check_errors_gl("glTranslatef");
}

#[allow(unused)] // todo: remove this
pub(crate) fn gl_rotate_f(angle: f32, x: f32, y: f32, z: f32) {
    unsafe { glRotatef(angle, x, y, z); }
    check_errors_gl("glRotatef");
}

pub(crate) fn gl_scale_f(x: f32, y: f32, z: f32) {
    unsafe { glScalef(x, y, z); }
    check_errors_gl("glScalef");
}

pub(crate) fn gl_polygon_mode(face: u32, mode: u32) {
    unsafe { glPolygonMode(face, mode) }
    check_errors_gl("glPolygonMode");
}
