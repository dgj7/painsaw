use windows::Win32::Foundation;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcA, DispatchMessageA, PeekMessageA, PostQuitMessage, TranslateMessage, MSG, PEEK_MESSAGE_REMOVE_TYPE};

pub(crate) fn peek_message(lpmsg: *mut MSG, hwnd: Option<Foundation::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> bool {
    unsafe { bool::from(PeekMessageA(lpmsg, hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg)) }
}

pub(crate) fn translate_message(lpmsg: *const MSG) -> bool {
    unsafe { bool::from(TranslateMessage(lpmsg)) }
}

pub(crate) fn dispatch_message(lpmsg: *const MSG) {
    unsafe { DispatchMessageA(lpmsg); }
}




pub(crate) fn validate_rect(hwnd: Option<Foundation::HWND>, lprect: Option<*const Foundation::RECT>) -> bool {
    unsafe { bool::from(ValidateRect(hwnd, lprect)) }
}

pub(crate) fn post_quit_message(nexitcode: i32) {
    unsafe { PostQuitMessage(nexitcode) }
}

pub(crate) fn default_window_proc(hwnd: Foundation::HWND, msg: u32, wparam: Foundation::WPARAM, lparam: Foundation::LPARAM) -> LRESULT {
    unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) }
}
