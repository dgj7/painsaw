use crate::math::twod::dimension_2d::Dimension2D;
use windows::Win32::Foundation;
use windows::Win32::Foundation::{LRESULT, RECT};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetWindowRect, LoadCursorW, PeekMessageW, PostQuitMessage, RegisterClassW, TranslateMessage, HCURSOR, HMENU, MSG, PEEK_MESSAGE_REMOVE_TYPE, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSW};
use crate::window::os::mswin::mswin_errors::check_errors_mswin;

pub(crate) fn peek_message(lpmsg: *mut MSG, hwnd: Option<Foundation::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> bool {
    let result = unsafe { bool::from(PeekMessageW(lpmsg, hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg)) };
    check_errors_mswin("PeekMessageW");
    result
}

pub(crate) fn translate_message(lpmsg: *const MSG) -> bool {
    let result = unsafe { bool::from(TranslateMessage(lpmsg)) };
    check_errors_mswin("TranslateMessage");
    result
}

pub(crate) fn dispatch_message(lpmsg: *const MSG) {
    unsafe { DispatchMessageW(lpmsg); }
    check_errors_mswin("DispatchMessageW");
}

pub(crate) fn get_module_handle<P>(lpmodulename: P) -> windows_core::Result<Foundation::HMODULE>
where
    P: windows_core::Param<windows_core::PCWSTR>,
{
    let result = unsafe { GetModuleHandleW(lpmodulename) };
    check_errors_mswin("GetModuleHandleW");
    result
}

pub(crate) fn load_cursor<P>(hinstance: Option<Foundation::HINSTANCE>, lpcursorname: P) -> windows_core::Result<HCURSOR>
where
    P: windows_core::Param<windows_core::PCWSTR>,
{
    let result = unsafe { LoadCursorW(hinstance, lpcursorname) };
    check_errors_mswin("LoadCursorW");
    result
}

pub(crate) fn register_class(lpwndclass: *const WNDCLASSW) -> u16 {
    let result = unsafe { RegisterClassW(lpwndclass) };
    check_errors_mswin("RegisterClassW");
    result
}

pub(crate) fn create_window_ex<P1, P2>(dwexstyle: WINDOW_EX_STYLE, lpclassname: P1, lpwindowname: P2, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<Foundation::HWND>, hmenu: Option<HMENU>, hinstance: Option<Foundation::HINSTANCE>, lpparam: Option<*const core::ffi::c_void>) -> windows_core::Result<Foundation::HWND>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    let result = unsafe { CreateWindowExW(dwexstyle, lpclassname, lpwindowname, dwstyle, x, y, nwidth, nheight, hwndparent, hmenu, hinstance, lpparam) };
    check_errors_mswin("CreateWindowExW");
    result
}

pub(crate) fn post_quit_message(nexitcode: i32) {
    unsafe { PostQuitMessage(nexitcode) }
    check_errors_mswin("PostQuitMessage");
}

pub(crate) fn default_window_proc(hwnd: Foundation::HWND, msg: u32, wparam: Foundation::WPARAM, lparam: Foundation::LPARAM) -> LRESULT {
    let result = unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    check_errors_mswin("DefWindowProcW");
    result
}

pub(crate) fn get_client_rect(hwnd: Foundation::HWND) -> Dimension2D {
    let rect = &mut RECT { left: 0, top: 0, right: 0, bottom: 0, };
    unsafe { GetClientRect(hwnd, rect) }.expect("TODO: get client rect");
    check_errors_mswin("GetClientRect");
    Dimension2D::new((rect.bottom - rect.top) as f32, (rect.right - rect.left) as f32)
}

pub(crate) fn get_window_rect(hwnd: Foundation::HWND) -> Dimension2D {
    let rect = &mut RECT { left: 0, top: 0, right: 0, bottom: 0, };
    unsafe { GetWindowRect(hwnd, rect) }.expect("TODO: get window rect");
    check_errors_mswin("GetWindowRect");
    Dimension2D::new((rect.bottom - rect.top) as f32, (rect.right - rect.left) as f32)
}
