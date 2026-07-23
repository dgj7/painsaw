use crate::window::os::mswin::winapi::{get_client_rect, get_cursor_pos, pt_in_rect};
use windows::Win32::Foundation::HWND;

pub fn is_mouse_over_window(hwnd: HWND) -> bool {
    let pos = get_cursor_pos();
    let rect = get_client_rect(hwnd);
    pt_in_rect(&rect, &pos)
}
