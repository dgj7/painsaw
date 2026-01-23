use crate::window::os::mswin::mswin_errors::check_errors_mswin;
use windows::Win32::Foundation;
use windows::Win32::Foundation::{LRESULT, RECT};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetWindowRect, LoadCursorW, PeekMessageW, PostQuitMessage, RegisterClassW, TranslateMessage, HCURSOR, HMENU, MSG, PEEK_MESSAGE_REMOVE_TYPE, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSW};
use crate::graphics::geometry::dim::Dimension2D;

pub(crate) fn peek_message(lpmsg: *mut MSG, hwnd: Option<Foundation::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> bool {
    unsafe { bool::from(PeekMessageW(lpmsg, hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg)) }
}

pub(crate) fn translate_message(lpmsg: *const MSG) -> bool {
    unsafe { bool::from(TranslateMessage(lpmsg)) }
}

pub(crate) fn dispatch_message(lpmsg: *const MSG) {
    unsafe { DispatchMessageW(lpmsg); }
}

pub(crate) fn get_module_handle<P>(lpmodulename: P) -> windows_core::Result<Foundation::HMODULE>
where
    P: windows_core::Param<windows_core::PCWSTR>,
{
    let result = unsafe { GetModuleHandleW(lpmodulename) };
    match &result {
        Ok(_) => {}
        Err(_) => check_errors_mswin("GetModuleHandleW")
    }
    result
}

pub(crate) fn load_cursor<P>(hinstance: Option<Foundation::HINSTANCE>, lpcursorname: P) -> windows_core::Result<HCURSOR>
where
    P: windows_core::Param<windows_core::PCWSTR>,
{
    let result = unsafe { LoadCursorW(hinstance, lpcursorname) };
    match &result {
        Ok(_) => {}
        Err(_) => check_errors_mswin("LoadCursorW")
    }
    result
}

pub(crate) fn register_class(lpwndclass: *const WNDCLASSW) -> u16 {
    let result = unsafe { RegisterClassW(lpwndclass) };
    if result <= 0 {
        check_errors_mswin("RegisterClassW");
    }
    result
}

pub(crate) fn create_window_ex<P1, P2>(dwexstyle: WINDOW_EX_STYLE, lpclassname: P1, lpwindowname: P2, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<Foundation::HWND>, hmenu: Option<HMENU>, hinstance: Option<Foundation::HINSTANCE>, lpparam: Option<*const core::ffi::c_void>) -> windows_core::Result<Foundation::HWND>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    let result = unsafe { CreateWindowExW(dwexstyle, lpclassname, lpwindowname, dwstyle, x, y, nwidth, nheight, hwndparent, hmenu, hinstance, lpparam) };
    match &result {
        Ok(_) => {},
        Err(_) => check_errors_mswin("CreateWindowExW")
    }
    result
}

pub(crate) fn post_quit_message(nexitcode: i32) {
    unsafe { PostQuitMessage(nexitcode) }
}

pub(crate) fn default_window_proc(hwnd: Foundation::HWND, msg: u32, wparam: Foundation::WPARAM, lparam: Foundation::LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

pub(crate) fn get_client_rect(hwnd: Foundation::HWND) -> Dimension2D<f32> {
    let rect = &mut RECT { left: 0, top: 0, right: 0, bottom: 0, };
    let result = unsafe { GetClientRect(hwnd, rect) };
    match result {
        Ok(_) => {}
        Err(_) => {check_errors_mswin("GetClientRect")}
    }
    Dimension2D::new((rect.bottom - rect.top) as f32, (rect.right - rect.left) as f32)
}

pub(crate) fn get_window_rect(hwnd: Foundation::HWND) -> Dimension2D<f32> {
    let rect = &mut RECT { left: 0, top: 0, right: 0, bottom: 0, };
    let result = unsafe { GetWindowRect(hwnd, rect) };
    match result {
        Ok(_) => {}
        Err(_) => {check_errors_mswin("GetWindowRect")}
    }
    Dimension2D::new((rect.bottom - rect.top) as f32, (rect.right - rect.left) as f32)
}
