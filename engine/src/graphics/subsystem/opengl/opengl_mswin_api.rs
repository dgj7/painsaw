use windows::Win32::Foundation;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC, HDC};
use windows::Win32::Graphics::OpenGL::{
    wglCreateContext, wglDeleteContext, wglGetCurrentContext, wglMakeCurrent, ChoosePixelFormat,
    SetPixelFormat, HGLRC, PIXELFORMATDESCRIPTOR,
};

pub fn wgl_delete_context(hrc: HGLRC) -> windows_core::Result<()> {
    unsafe { wglDeleteContext(hrc) }
}

pub fn release_dc(hwnd: Option<HWND>, hdc: HDC) -> i32 {
    unsafe { ReleaseDC(hwnd, hdc) }
}

pub fn get_dc(hwnd: Option<Foundation::HWND>) -> HDC {
    unsafe { GetDC(hwnd) }
}

pub fn choose_pixel_format(hdc: HDC, pfd: *const PIXELFORMATDESCRIPTOR) -> i32 {
    unsafe { ChoosePixelFormat(hdc, pfd) }
}

pub fn set_pixel_format(hdc: HDC, format: i32, pfd: *const PIXELFORMATDESCRIPTOR) -> windows_core::Result<()> {
    unsafe { SetPixelFormat(hdc, format, pfd) }
}

pub fn wgl_create_context(hdc: HDC) -> windows_core::Result<HGLRC> {
    unsafe { wglCreateContext(hdc) }
}

pub fn wgl_get_current_context() -> HGLRC {
    unsafe { wglGetCurrentContext() }
}

pub fn wgl_make_current(hdc: HDC, hglrc: HGLRC) -> windows_core::Result<()> {
    unsafe { wglMakeCurrent(hdc, hglrc) }
}
