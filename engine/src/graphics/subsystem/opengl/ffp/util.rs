use crate::graphics::subsystem::opengl::ffp::api::gl_begin;
use windows::Win32::Graphics::OpenGL::{GL_LINES, GL_POINTS, GL_QUADS, GL_LINE_STRIP};

pub(crate) fn gl_begin_lines() {
    gl_begin(GL_LINES);
}

pub(crate) fn gl_begin_line_strip() {
    gl_begin(GL_LINE_STRIP);
}

pub(crate) fn gl_begin_points() {
    gl_begin(GL_POINTS);
}

pub(crate) fn gl_begin_quads() {
    gl_begin(GL_QUADS);
}
