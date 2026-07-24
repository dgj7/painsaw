use crate::window::os::mswin::errors::check_errors_mswin;
use windows::Win32::Foundation;
use windows::Win32::Foundation::{LRESULT, POINT, RECT};
use windows::Win32::Graphics::Gdi::PtInRect;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::{RegisterRawInputDevices, RAWINPUTDEVICE};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetCursorPos, GetWindowRect, LoadCursorW, PeekMessageW, PostQuitMessage, RegisterClassW, TranslateMessage, HCURSOR, HMENU, MSG, PEEK_MESSAGE_REMOVE_TYPE, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSW};

///
/// PeekMessageW()
///
pub(crate) fn peek_message(lpmsg: *mut MSG, hwnd: Option<Foundation::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> bool {
    unsafe { bool::from(PeekMessageW(lpmsg, hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg)) }
}

///
/// TranslateMessage()
///
pub(crate) fn translate_message(lpmsg: *const MSG) -> bool {
    unsafe { bool::from(TranslateMessage(lpmsg)) }
}

///
/// DispatchMessageW()
///
pub(crate) fn dispatch_message(lpmsg: *const MSG) {
    unsafe { DispatchMessageW(lpmsg); }
}

///
/// GetModuleHandleW()
///
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

///
/// LoadCursorW()
///
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

///
/// RegisterClassW()
///
pub(crate) fn register_class(lpwndclass: *const WNDCLASSW) -> u16 {
    let result = unsafe { RegisterClassW(lpwndclass) };
    if result <= 0 {
        check_errors_mswin("RegisterClassW");
    }
    result
}

///
/// CreateWindowExW()
///
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

///
/// PostQuitMessage()
///
pub(crate) fn post_quit_message(nexitcode: i32) {
    unsafe { PostQuitMessage(nexitcode) }
}

///
/// DefWindowProcW()
///
pub(crate) fn default_window_proc(hwnd: Foundation::HWND, msg: u32, wparam: Foundation::WPARAM, lparam: Foundation::LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

///
/// GetClientRect()
///
pub(crate) fn get_client_rect(hwnd: Foundation::HWND) -> RECT {
    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0, };
    let result = unsafe { GetClientRect(hwnd, &mut rect) };
    match result {
        Ok(_) => {}
        Err(_) => {check_errors_mswin("GetClientRect")}
    }
    rect
}

///
/// GetWindowRect()
///
#[allow(unused)]
pub(crate) fn get_window_rect(hwnd: Foundation::HWND) -> RECT {
    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0, };
    let result = unsafe { GetWindowRect(hwnd, &mut rect) };
    match result {
        Ok(_) => {}
        Err(_) => {check_errors_mswin("GetWindowRect")}
    }
    rect
}

///
/// GetCursorPos()
///
pub(crate) fn get_cursor_pos() -> POINT {
    let mut pt = POINT { x: 0, y: 0 };
    let result = unsafe { GetCursorPos(&mut pt) };
    match result {
        Ok(_) => {}
        Err(_) => {check_errors_mswin("GetCursorPos")}
    }
    pt
}

///
/// PtInRect()
///
pub(crate) fn pt_in_rect(rect: &RECT, pt: &POINT) -> bool {
    unsafe { bool::from(PtInRect(rect, *pt)) }
}

///
/// RegisterRawInputDevices()
///
pub(crate) fn register_raw_input_devices(rid: &[RAWINPUTDEVICE]) {
    let result = unsafe { RegisterRawInputDevices(rid, std::mem::size_of::<RAWINPUTDEVICE>() as u32) };
    match result {
        Ok(_) => {}
        Err(_) => {check_errors_mswin("RegisterRawInputDevices")}
    }
}
