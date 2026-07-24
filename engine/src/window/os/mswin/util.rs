use crate::geometry::dim::Dimension2D;
use crate::window::os::mswin::winapi::{get_client_rect, get_cursor_pos, pt_in_rect};
use windows::Win32::Foundation::HWND;

///
/// Determine if the mouse is hovering over the window.
///
pub fn is_mouse_over_window(hwnd: HWND) -> bool {
    let pos = get_cursor_pos();
    let rect = get_client_rect(hwnd);
    pt_in_rect(&rect, &pos)
}

///
/// get the client [RECT] as [Dimension2D].
///
/// calls GetWindowRect() (win32) internally.
///
pub(crate) fn get_client_rect_dim2d(hwnd: HWND) -> Dimension2D {
    let rect = get_client_rect(hwnd);
    Dimension2D::new((rect.bottom - rect.top) as f32, (rect.right - rect.left) as f32)
}

///
/// get the window [RECT] as [Dimension2D].
///
/// calls GetWindowRect() (win32) internally.
///
pub(crate) fn get_window_rect_dim2d(hwnd: HWND) -> Dimension2D {
    let rect = get_client_rect(hwnd);
    Dimension2D::new((rect.bottom - rect.top) as f32, (rect.right - rect.left) as f32)
}
