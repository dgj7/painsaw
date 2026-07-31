use std::error::Error;
use std::panic;
use std::panic::AssertUnwindSafe;
use crate::config::EngineConfig;
use crate::window::mswin::MsWinWindow;
use crate::window::Window;
use crate::window::error::WindowingError;

///
/// Create a window.
///
/// Behavior changes depending on operating system.
///
/// See also: https://doc.rust-lang.org/reference/conditional-compilation.html
///
pub fn create_window(request: &EngineConfig) -> Result<Box<dyn Window>, Box<dyn Error>> {
    create_window_os(request)
}

///
/// Create a window for Microsoft Windows.
///
#[cfg(target_os="windows")]
fn create_window_os(request: &EngineConfig) -> Result<Box<dyn Window>, Box<dyn Error>> {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        return MsWinWindow::new(request);
    }));
    result.unwrap_or_else(|_err| Err(WindowingError(String::from("TODO: gather error info here")).into()))
}

///
/// Create a window for Linux.
///
#[cfg(target_os="linux")]
fn create_window_os(request: &EngineConfig) -> Result<Box<dyn Window>, Box<dyn Error>> {
    todo!("linux windowing not yet implemented")
}

///
/// Create a window for MacOS.
///
#[cfg(target_os="macos")]
fn create_window_os(request: &EngineConfig) -> Result<Box<dyn Window>, Box<dyn Error>> {
    todo!("macos windowing not yet implemented")
}