use glcore::{GL_LINES, GL_LINE_STRIP, GL_POINTS, GL_QUADS, GL_TRIANGLES};
use crate::graphics::subsystem::opengl::ffp::api::gl_begin;

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

pub(crate) fn gl_begin_triangles() {
    gl_begin(GL_TRIANGLES);
}
