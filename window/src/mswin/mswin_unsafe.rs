use windows::Win32::Foundation;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExA, DefWindowProcA, DispatchMessageA, LoadCursorW, PeekMessageA, PostQuitMessage, RegisterClassA, TranslateMessage, HCURSOR, HMENU, MSG, PEEK_MESSAGE_REMOVE_TYPE, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSA};

pub(crate) fn peek_message(lpmsg: *mut MSG, hwnd: Option<Foundation::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> bool {
    unsafe { bool::from(PeekMessageA(lpmsg, hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg)) }
}

pub(crate) fn translate_message(lpmsg: *const MSG) -> bool {
    unsafe { bool::from(TranslateMessage(lpmsg)) }
}

pub(crate) fn dispatch_message(lpmsg: *const MSG) {
    unsafe { DispatchMessageA(lpmsg); }
}

pub(crate) fn get_module_handle<P>(lpmodulename: P) -> windows_core::Result<Foundation::HMODULE>
where
    P: windows_core::Param<windows_core::PCSTR>,
{
    unsafe { GetModuleHandleA(lpmodulename) }
}

pub(crate) fn load_cursor<P>(hinstance: Option<Foundation::HINSTANCE>, lpcursorname: P) -> windows_core::Result<HCURSOR>
where
    P: windows_core::Param<windows_core::PCWSTR>,
{
    unsafe { LoadCursorW(hinstance, lpcursorname) }
}

pub(crate) fn register_class(lpwndclass: *const WNDCLASSA) -> u16 {
    unsafe { RegisterClassA(lpwndclass) }
}

pub(crate) fn create_window_ex<P1, P2>(dwexstyle: WINDOW_EX_STYLE, lpclassname: P1, lpwindowname: P2, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<Foundation::HWND>, hmenu: Option<HMENU>, hinstance: Option<Foundation::HINSTANCE>, lpparam: Option<*const core::ffi::c_void>) -> windows_core::Result<Foundation::HWND>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    unsafe { CreateWindowExA(dwexstyle, lpclassname, lpwindowname, dwstyle, x, y, nwidth, nheight, hwndparent, hmenu, hinstance, lpparam) }
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
