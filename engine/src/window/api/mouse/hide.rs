use crate::window::mswin::winapi::show_cursor;

#[allow(unused)] // todo
pub fn hide_mouse() {
    hide_mouse_os()
}

#[cfg(target_os = "windows")]
fn hide_mouse_os() {
    while show_cursor(false) >= 0 {}
}

#[cfg(target_os = "linux")]
fn hide_mouse_os() {
    todo!("linux")
}

#[cfg(target_os = "macos")]
fn hide_mouse_os() {
    todo!("macos")
}