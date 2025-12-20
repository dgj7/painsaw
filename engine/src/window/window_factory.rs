use std::error::Error;
use std::panic;
use std::sync::Arc;
use crate::logger::Logger;
use crate::window::model::window_config::WindowConfig;
use crate::window::model::window_error::WindowError;
use crate::window::os::mswin::mswin_window::MsWinWindow;
use crate::window::window::Window;

///
/// Create a window.
///
/// Behavior changes depending on operating system.
///
/// See also: https://doc.rust-lang.org/reference/conditional-compilation.html
///
pub fn create_window(request: &WindowConfig, logger: &Arc<Logger>) -> Result<Box<dyn Window>, Box<dyn Error>> {
    create_window_os(request, logger)
}

///
/// Create a window for Microsoft Windows.
///
#[cfg(target_os="windows")]
fn create_window_os(request: &WindowConfig, logger: &Arc<Logger>) -> Result<Box<dyn Window>, Box<dyn Error>> {
    let result = panic::catch_unwind(|| {
        return MsWinWindow::new(request, logger);
    });
    result.unwrap_or_else(|_err| Err(WindowError(String::from("TODO: gather error info here")).into()))
}

///
/// Create a window for Linux.
///
#[cfg(target_os="linux")]
fn create_window_os(request: &WindowConfig, logger: &Arc<LoggerConfig>) -> Result<Box<dyn Window>, Box<dyn Error>> {
    todo!("linux windowing not yet implemented")
}

///
/// Create a window for MacOS.
///
#[cfg(target_os="macos")]
fn create_window_os(request: &WindowConfig, logger: &Arc<LoggerConfig>) -> Result<Box<dyn Window>, Box<dyn Error>> {
    todo!("macos windowing not yet implemented")
}
