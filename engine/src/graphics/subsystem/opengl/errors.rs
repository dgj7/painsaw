use crate::support::logger::log_level::LogLevel;
use crate::support::logger::log_caller;
use std::ffi::{c_uchar, CStr};
use std::panic::Location;
use windows::Win32::Graphics::OpenGL::{glGetError, gluErrorString, GL_NO_ERROR};

#[track_caller]
pub fn check_errors_gl(caller: &str) {
    let code = gl_get_error();
    if code != GL_NO_ERROR {
        let message = glu_error_string(code);
        log_caller(LogLevel::Error, Location::caller(), &|| String::from(format!("GL_ERROR: {}: {}: {}", caller, code, message)));
    }
}

fn glu_error_string(code: u32) -> String {
    let ptr: *const c_uchar = unsafe { gluErrorString(code) };
    if ptr.is_null() {
        return "unknown".to_string();
    }

    let cstr = unsafe { CStr::from_ptr(ptr as *const i8) };
    let result = match cstr.to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => "invalid".to_string(),
    };

    result
}

fn gl_get_error() -> u32 {
    unsafe { glGetError() }
}
