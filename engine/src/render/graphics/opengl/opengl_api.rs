use windows::Win32::Graphics::OpenGL::{glBegin, glClear, glClearColor, glColor3f, glEnd, glFlush, glLineWidth, glVertex2f, GL_LINES};

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

pub fn gl_line_width(width_pixels: f32) {
    unsafe { glLineWidth(width_pixels); }
}

pub fn gl_color_3f(red: f32, green: f32, blue: f32) {
    unsafe { glColor3f(red, green, blue); }
}

pub fn gl_vertex_2f(x: f32, y: f32) {
    unsafe { glVertex2f(x, y); }
}
