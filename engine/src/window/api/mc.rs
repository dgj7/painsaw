use crate::geometry::primitive::v2d::Vertex2D;
use crate::window::mswin::winapi::set_cursor_pos;

pub fn move_cursor(destination: &Vertex2D) {
    move_cursor_os(destination);
}

#[cfg(target_os="windows")]
fn move_cursor_os(destination: &Vertex2D) {
    set_cursor_pos(destination.x as i32, destination.y as i32);
}

#[cfg(target_os="linux")]
fn move_cursor_os(destination: Vertex2D) {
    todo!("linux windowing not yet implemented")
}

#[cfg(target_os="macos")]
fn move_cursor_os(destination: Vertex2D) {
    todo!("macos windowing not yet implemented")
}
