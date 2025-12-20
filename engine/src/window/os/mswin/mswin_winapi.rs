use windows::Win32::Foundation;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Graphics::Gdi::{GetDC, HDC};
use windows::Win32::Graphics::OpenGL::{wglCreateContext, wglMakeCurrent, ChoosePixelFormat, SetPixelFormat, HGLRC, PIXELFORMATDESCRIPTOR};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, LoadCursorW, PeekMessageW, PostQuitMessage, RegisterClassW, TranslateMessage, HCURSOR, HMENU, MSG, PEEK_MESSAGE_REMOVE_TYPE, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSW};

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
    unsafe { GetModuleHandleW(lpmodulename) }
}

pub(crate) fn load_cursor<P>(hinstance: Option<Foundation::HINSTANCE>, lpcursorname: P) -> windows_core::Result<HCURSOR>
where
    P: windows_core::Param<windows_core::PCWSTR>,
{
    unsafe { LoadCursorW(hinstance, lpcursorname) }
}

pub(crate) fn register_class(lpwndclass: *const WNDCLASSW) -> u16 {
    unsafe { RegisterClassW(lpwndclass) }
}

pub(crate) fn create_window_ex<P1, P2>(dwexstyle: WINDOW_EX_STYLE, lpclassname: P1, lpwindowname: P2, dwstyle: WINDOW_STYLE, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<Foundation::HWND>, hmenu: Option<HMENU>, hinstance: Option<Foundation::HINSTANCE>, lpparam: Option<*const core::ffi::c_void>) -> windows_core::Result<Foundation::HWND>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    unsafe { CreateWindowExW(dwexstyle, lpclassname, lpwindowname, dwstyle, x, y, nwidth, nheight, hwndparent, hmenu, hinstance, lpparam) }
}

pub(crate) fn post_quit_message(nexitcode: i32) {
    unsafe { PostQuitMessage(nexitcode) }
}

pub(crate) fn default_window_proc(hwnd: Foundation::HWND, msg: u32, wparam: Foundation::WPARAM, lparam: Foundation::LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

pub(crate) fn get_dc(hwnd: Option<Foundation::HWND>) -> HDC {
    unsafe { GetDC(hwnd) }
}

pub(crate) fn choose_pixel_format(hdc: HDC, pfd: *const PIXELFORMATDESCRIPTOR) -> i32 {
    unsafe { ChoosePixelFormat(hdc, pfd) }
}

pub(crate) fn set_pixel_format(hdc: HDC, format: i32, pfd: *const PIXELFORMATDESCRIPTOR) -> windows_core::Result<()> {
    unsafe { SetPixelFormat(hdc, format, pfd) }
}

pub(crate) fn wgl_create_context(hdc: HDC) -> windows_core::Result<HGLRC> {
    unsafe { wglCreateContext(hdc) }
}

pub(crate) fn wgl_make_current(hdc: HDC, hglrc: HGLRC) -> windows_core::Result<()> {
    unsafe { wglMakeCurrent(hdc, hglrc) }
}

pub(crate) fn wgl_make_current_cleanup() {
    wgl_make_current(HDC(std::ptr::null_mut()), HGLRC(std::ptr::null_mut())).expect("TODO: panic message");
}
