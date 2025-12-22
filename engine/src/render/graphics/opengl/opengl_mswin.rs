use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::OpenGL::{SwapBuffers, HGLRC, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR};
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::render::graphics::opengl::opengl_mswin_api::{choose_pixel_format, get_dc, set_pixel_format, wgl_create_context, wgl_make_current};

pub fn init_opengl(hwnd: HWND) -> (HDC, HGLRC) {
    /* opengl: create pixel format */
    let pfd = PIXELFORMATDESCRIPTOR {
        nSize: size_of::<PIXELFORMATDESCRIPTOR>() as _,
        nVersion: 1,
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 32,
        cRedBits: 0,
        cRedShift: 0,
        cGreenBits: 0,
        cGreenShift: 0,
        cBlueBits: 0,
        cBlueShift: 0,
        cAlphaBits: 0,
        cAlphaShift: 0,
        cAccumBits: 0,
        cAccumRedBits: 0,
        cAccumGreenBits: 0,
        cAccumBlueBits: 0,
        cAccumAlphaBits: 0,
        cDepthBits: 24,
        cStencilBits: 0,
        cAuxBuffers: 0,
        iLayerType: PFD_MAIN_PLANE.0 as u8, // PFD_MAIN_PLANE is negative i8, 0 as u8
        bReserved: 0,
        dwLayerMask: 0,
        dwVisibleMask: 0,
        dwDamageMask: 0,
    };

    /* opengl: get the device context */
    let hdc = get_dc(Option::from(hwnd));

    /* opengl: set the pixel format */
    let pf = choose_pixel_format(hdc, &pfd);
    set_pixel_format(hdc, pf, &pfd).expect("todo: set pixel format");

    /* opengl: get hrc */
    let hrc = wgl_create_context(hdc).expect("todo: wgl create context");
    wgl_make_current(hdc, hrc).expect("todo: wgl make current");

    /* log the things */
    log(LogLevel::Debug, &|| String::from(format!("init_opengl(): hdc={:?}, hrc={:?}", hdc, hrc)));

    /* done */
    (hdc, hrc)
}

pub fn swap_buffers(hdc: HDC) {
    unsafe { SwapBuffers(hdc).expect("TODO: swap buffers"); }
}
