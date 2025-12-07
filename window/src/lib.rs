use std::panic;
use crate::model::window::Window;
use crate::model::window_error::WindowError;
use crate::model::window_request::WindowRequest;
use crate::mswin::mswin_window::MsWinWindow;

pub mod model;
pub mod mswin;

///
/// Create a window.
///
/// Behavior changes depending on operating system.
///
/// See also: https://doc.rust-lang.org/reference/conditional-compilation.html
///
pub fn create_window(request: &WindowRequest) -> Result<Box<dyn Window>, WindowError> {
    create_window_os(request)
}

///
/// Create a window for Microsoft Windows.
///
#[cfg(target_os="windows")]
fn create_window_os(request: &WindowRequest) -> Result<Box<dyn Window>, WindowError> {
    let result = panic::catch_unwind(|| {
        return MsWinWindow::new(request);
    });
    match result {
        Ok(window) => Ok(window),
        Err(_err) => Err(WindowError(String::from("TODO: gather error info here"))),
    }
}

///
/// Create a window for Linux.
///
#[cfg(target_os="linux")]
fn create_window_os(request: &WindowRequest) -> Result<Box<dyn Window>, WindowError> {
    todo!("linux windowing not yet implemented")
}

///
/// Create a window for MacOS.
///
#[cfg(target_os="macos")]
fn create_window_os(request: &WindowRequest) -> Result<Box<dyn Window>, WindowError> {
    todo!("macos windowing not yet implemented")
}
