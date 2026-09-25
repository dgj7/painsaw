use crate::window::mswin::winapi::show_cursor;

pub fn toggle_cursor_visibility() {
    toggle_cursor_visibility_os()
}

#[cfg(target_os = "windows")]
fn toggle_cursor_visibility_os() {
    let counter = show_cursor(false);

    if counter < -1 {
        show_cursor(true);
        show_cursor(true);
    }
}

#[cfg(target_os = "linux")]
fn toggle_cursor_visibility_os() {
    todo!("linux")
}

#[cfg(target_os = "macos")]
fn toggle_cursor_visibility_os() {
    todo!("macos")
}
