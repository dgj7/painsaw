use windows::Win32::Graphics::OpenGL::{glClear, glClearColor};

pub fn gl_clear(mask: u32) {
    unsafe { glClear(mask); }
}

pub fn gl_clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { glClearColor(red, green, blue, alpha); }
}
