use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::OpenGL::HGLRC;
use windows::Win32::UI::WindowsAndMessaging::WNDCLASSW;

///
/// uniquely identify a window, regardless what operating system.
/// 
pub struct WindowKey {
    /* Microsoft Windows */
    #[cfg(target_os="windows")]
    pub hinstance: HINSTANCE,
    #[cfg(target_os="windows")]
    pub wndclassw: WNDCLASSW,
    #[cfg(target_os="windows")]
    pub atom: u16,
    #[cfg(target_os="windows")]
    pub hwnd: HWND,
    #[cfg(target_os="windows")]
    pub hdc: HDC,
    #[cfg(target_os="windows")]
    pub hrc: HGLRC,
}
