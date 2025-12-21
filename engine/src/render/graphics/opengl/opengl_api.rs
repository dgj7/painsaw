use windows::Win32::Graphics::OpenGL::{glBegin, glClear, glClearColor, glColor3f, glEnd, glFlush, glLineWidth, glLoadIdentity, glMatrixMode, glOrtho, glVertex2f, glViewport, GL_LINES};

pub fn gl_clear(mask: u32) {
    unsafe { glClear(mask); }
}

pub fn gl_clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glClearColor(red, green, blue, alpha); }
}

pub fn gl_begin_lines() {
    unsafe { glBegin(GL_LINES); }
}

pub fn gl_end() {
    unsafe { glEnd(); }
}

pub fn gl_flush() {
    unsafe { glFlush(); }
}

pub fn gl_viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { glViewport(x, y, width, height); }
}

pub fn gl_matrix_mode(mode: u32) {
    unsafe { glMatrixMode(mode) }
}

pub fn gl_load_identity() {
    unsafe { glLoadIdentity() }
}

pub fn gl_ortho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    unsafe { glOrtho(left, right, bottom, top, znear, zfar) }
}

pub fn gl_line_width(width_pixels: f32) {
    unsafe { glLineWidth(width_pixels); }
}

pub fn gl_color_3f(red: f32, green: f32, blue: f32) {
    unsafe { glColor3f(red, green, blue); }
}

pub fn gl_vertex_2f(x: f32, y: f32) {
    unsafe { glVertex2f(x, y); }
}
