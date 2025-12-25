use crate::input::model::input_state::InputState;
use crate::input::model::keyboard_state::{KeyInfo, KeyPosition};
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::render::graphics::opengl::opengl_mswin::{init_opengl, swap_buffers};
use crate::render::graphics::opengl::opengl_mswin_api::{get_dc, release_dc, wgl_delete_context, wgl_get_current_context, wgl_make_current};
use crate::render::model::render_context::RendererContext;
use crate::render::renderer::Renderer;
use crate::window::model::window_config::{WindowConfig, WindowDimensions};
use crate::window::os::mswin::mswin_data::{create_and_write_pointer, input_state_to_raw_pointer, read_window_data};
use crate::window::os::mswin::mswin_winapi::{create_window_ex, default_window_proc, dispatch_message, get_client_rect, get_module_handle, get_window_rect, load_cursor, peek_message, post_quit_message, register_class, translate_message};
use crate::window::window::Window;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::OpenGL::*;
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_ESCAPE, VK_G};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub struct MsWinWindow {
    pub input: Arc<Mutex<InputState>>,

    pub hinstance: HINSTANCE,
    pub wndclassw: WNDCLASSW,
    pub atom: u16,
    pub hwnd: HWND,
    pub quit: bool,

    pub hdc: HDC,
    pub hrc: HGLRC,
}

impl Window for MsWinWindow {
    fn begin_event_handling(&mut self, renderer: &dyn Renderer) -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        log(LogLevel::Info, &|| "begin event handling".parse().unwrap());
        let mut message: MSG = MSG::default();
        let mut context = RendererContext::new(&self.input);

        while !self.quit {
            if peek_message(&mut message, Default::default(), 0, 0, PM_REMOVE) {
                if message.message == WM_QUIT {
                    log(LogLevel::Debug, &|| String::from("WM_QUIT"));
                    self.quit = true;
                    opengl_cleanup(self.hwnd);
                    break;
                }

                let _ = translate_message(&message);
                dispatch_message(&message);
            } else {
                /* update the demo1 world */
                renderer.update_world(&mut context);

                /* render the demo1 world */
                renderer.before_render(&mut context);
                renderer.render_3d_scene(&mut context);
                renderer.render_2d_scene(&mut context);
                renderer.after_render(&mut context);

                /* swap buffers after it's all done */
                swap_buffers(self.hdc);
            }
        }

        log(LogLevel::Info, &|| { return String::from(format!("after while(!quit); rendered {} frames", context.frame_count)) });

        Ok(())
    }

    fn get_input_state(&self) -> Arc<Mutex<InputState>> {
        self.input.clone()
    }
}

impl MsWinWindow {
    pub fn new(request : &WindowConfig) -> std::result::Result<Box<dyn Window>, Box<dyn std::error::Error>> {
        /* make some variables */
        let wndclass = PCWSTR::from_raw(HSTRING::from(request.wndclass.clone().unwrap_or(String::from("WindowConfig: set wndclass"))).as_ptr());
        let title = PCWSTR::from_raw(HSTRING::from(request.title.clone().unwrap_or(String::from("WindowConfig: set title"))).as_ptr());

        /* get handle instance */
        let hinstance: HINSTANCE = HINSTANCE::from(get_module_handle(None)?);
        debug_assert!(hinstance.0 != std::ptr::null_mut());

        /* create the wnd class */
        let wc = WNDCLASSW {
            hCursor: load_cursor(None, IDC_ARROW)?,
            hbrBackground: Default::default(),
            hInstance: hinstance,
            lpszClassName: wndclass,
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: Default::default(),
            lpszMenuName: Default::default(),
        };

        /* register the wnd class */
        let atom = register_class(&wc);
        debug_assert!(atom != 0);

        /* determine some settings based on configuration */
        let dwstyle = match request.dimensions {
            WindowDimensions::Fullscreen => WS_VISIBLE,
            WindowDimensions::Dimensional { width: _width, height: _height } => WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_THICKFRAME,
        };
        let (x, y) = match request.dimensions {
            WindowDimensions::Fullscreen => (0, 0),
            WindowDimensions::Dimensional { width: _width, height: _height } => (CW_USEDEFAULT, CW_USEDEFAULT),
        };
        let (width,height) = match request.dimensions {
            WindowDimensions::Fullscreen => (CW_USEDEFAULT, CW_USEDEFAULT),
            WindowDimensions::Dimensional { width, height } => (width, height),
        };

        /* create input state */
        let input = InputState::new();
        let input_pointer = input_state_to_raw_pointer(&input);

        /* create the window */
        let hwnd = create_window_ex(
            WINDOW_EX_STYLE::default(),
            wndclass,
            title,
            dwstyle,
            x,
            y,
            width,
            height,
            None,                                               // no parent window
            None,                                               // no menus
            Option::from(hinstance),
            Some(input_pointer),
        ).expect("CreateWindowEx* failed");

        /* init opengl */
        let (hdc, hrc) = init_opengl(hwnd);

        /* done; returning handles to window */
        Ok(Box::new(MsWinWindow {
            input,
            hinstance,
            wndclassw: wc,
            atom,
            hwnd,
            quit: false,
            hdc,
            hrc,
        }))
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_CREATE => {// 0x0001: sent when createwindowex/createwindow is called; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create
            create_and_write_pointer(window, lparam);
            LRESULT(0)
        }
        WM_DESTROY => {
            log(LogLevel::Debug, &|| String::from("WM_DESTROY"));

            let input = read_window_data(window).unwrap();
            drop(input);

            post_quit_message(0);

            LRESULT(0)
        }
        //WM_MOVING => {// 0x0216: when user is moving the window; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-moving
        //    LRESULT(0)
        //}
        //WM_MOVE => {// 0x0003: after a window has been moved; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-move
        //    LRESULT(0)
        //}
        //WM_NCMOUSELEAVE => {// 0x02A2: curor leaves the nonclinent area of the window; https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-ncmouseleave
        //    LRESULT(0)
        //}
        //WM_GETMINMAXINFO => {// 0x0024: size/pos of the window is about to change; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo
        //    LRESULT(0)
        //}
        //WM_WINDOWPOSCHANGING => {// 0x0046: size/pos/place (z-order) is about to change, from SetWindowPos; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging
        //    LRESULT(0)
        //}
        //WM_WINDOWPOSCHANGED => {// 0x0047: size/pos/place (z-order) has changed, from SetWindowPos; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged
        //    LRESULT(0)
        //}
        _ => {
            let maybe_input = read_window_data(window);
            if let Some(input) = maybe_input {
                let handled = handle_message_if_applicable(&input, window, message, wparam, lparam);
                if handled {
                    LRESULT::default()
                } else {
                    default_window_proc(window, message, wparam, lparam)
                }
            } else {
                default_window_proc(window, message, wparam, lparam)
            }
        }
    }
}

fn handle_message_if_applicable(input: &Arc<Mutex<InputState>>, hwnd: HWND, message: u32, wparam: WPARAM, _lparam: LPARAM) -> bool {
    match message {
        WM_KEYDOWN => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_ESCAPE => {
                    post_quit_message(0);
                    true
                }
                VK_G => {
                    match input.lock() {
                        Ok(mut is) => {is.g_key.update(KeyPosition::KeyDown { info: KeyInfo { when: Instant::now(), handled: false } })}
                        Err(_) => panic!("todo: g: down")
                    }
                    true
                }
                _ => true
            }
        }
        WM_KEYUP => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_G => {
                    match input.lock() {
                        Ok(mut is) => {is.g_key.update(KeyPosition::KeyUp { info: KeyInfo { when: Instant::now(), handled: false } })}
                        Err(_) => panic!("todo: g: up")
                    }
                    true
                }
                _ => true
            }
        }
        WM_SIZE => {
            // see also: WM_SIZING: while the user is actively resizing the window
            // see also: WM_ENTERSIZEMOVE: resizing started
            // see also: WM_EXITSIZEMOVE: resizing ended
            let window_dimensions = get_window_rect(hwnd);
            let client_dimensions = get_client_rect(hwnd);
            log(LogLevel::Trace, &|| format!("window_dimensions {:?}", window_dimensions));
            log(LogLevel::Trace, &|| format!("client_dimensions {:?}", client_dimensions));
            match input.lock() {
                Ok(mut is) => {
                    is.update_client_dimensions(client_dimensions);
                    is.update_window_dimensions(window_dimensions);
                }
                Err(_) => panic!("todo: wm_size")
            }
            true
        }
        _ => false
    }
}

fn opengl_cleanup(hwnd: HWND) {
    /* retrieve required handles */
    let hdc = get_dc(Option::from(hwnd));
    let hrc = wgl_get_current_context();

    /* log the values in case we need to compare them at some point */
    log(LogLevel::Debug, &|| String::from(format!("opengl cleanup: hdc={:?}, hrc={:?}", hdc, hrc)));

    /* do the cleanup */
    wgl_make_current(HDC(std::ptr::null_mut()), HGLRC(std::ptr::null_mut())).expect("TODO: cleanup: make current failed");
    wgl_delete_context(hrc).expect("TODO: cleanup: delete context failed");
    release_dc(Option::from(hwnd), hdc);

    /* declare success */
    log(LogLevel::Trace, &|| String::from("opengl cleanup completed"));
}
