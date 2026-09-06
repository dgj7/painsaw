use crate::window::mswin::winapi::show_cursor;

#[allow(unused)] // todo
pub fn show_mouse() {
    show_mouse_os()
}

#[cfg(target_os = "windows")]
fn show_mouse_os() {
    while show_cursor(true) <= 1 {}
}

#[cfg(target_os = "linux")]
fn show_mouse_os() {
    todo!("linux")
}

#[cfg(target_os = "macos")]
fn show_mouse_os() {
    todo!("macos")
}
