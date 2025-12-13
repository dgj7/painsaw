use std::time::Instant;
use crate::model::window::Window;
use crate::model::window_config::{WindowConfig, WindowDimensions};
use crate::mswin::mswin_unsafe::{create_window_ex, default_window_proc, dispatch_message, get_module_handle, load_cursor, peek_message, post_quit_message, register_class, translate_message};
use logger::model::log_config::LoggerConfig;
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_ESCAPE, VK_G};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};
use crate::input::InputState;
use crate::input::keyboard_state::KeyState::{KeyDown, KeyUp};
use crate::render::render_context::RendererContext;
use crate::render::renderer::Renderer;

pub struct MsWinWindow {
    pub input: InputState,

    pub hinstance: HINSTANCE,
    pub wndclassa: WNDCLASSA,
    pub atom: u16,
    pub hwnd: HWND,
    pub quit: bool,
}

impl Window for MsWinWindow {
    fn begin_event_handling(&mut self, renderer: &dyn Renderer, logger: &LoggerConfig) -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        logger.debug(&|| "begin event handling");
        let mut message: MSG = MSG::default();
        let mut context = RendererContext::new();

        while !self.quit {
            if peek_message(&mut message, Default::default(), 0, 0, PM_REMOVE) {
                if message.message == WM_QUIT {
                    self.quit = true;
                    break;
                }

                let _ = translate_message(&message);
                dispatch_message(&message);
            } else {
                renderer.render_scene(&mut context);
            }
        }

        logger.info(&|| "after while(!quit)");
        Ok(())
    }

    fn get_input_state(&self) -> &InputState {
        &self.input
    }
}

impl MsWinWindow {
    pub fn new(request : &WindowConfig) -> std::result::Result<Box<dyn Window>, Box<dyn std::error::Error>> {
        /* get handle instance */
        let hinstance: HINSTANCE = HINSTANCE::from(get_module_handle(None)?);
        debug_assert!(hinstance.0 != std::ptr::null_mut());

        /* create the wnd class */
        let wc = WNDCLASSA {
            hCursor: load_cursor(None, IDC_ARROW)?,
            hInstance: hinstance,
            lpszClassName: s!("window"),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
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

        /* create the window */
        let hwnd = create_window_ex(
            WINDOW_EX_STYLE::default(),
            s!("window"),
            s!("This is a sample window"),
            dwstyle,
            x,
            y,
            width,
            height,
            None,                                               // no parent window
            None,                                               // no menus
            Option::from(hinstance),
            Some(&mut input.to_owned() as *mut _ as _),
        ).expect("CreateWindowEx* failed");

        /* done; returning handles to window so we can create device context later */
        Ok(Box::new(MsWinWindow {
            input,
            hinstance,
            wndclassa: wc,
            atom,
            hwnd,
            quit: false,
        }))
    }
}

// https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/direct3d12/src/main.rs
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_CREATE => {
            // todo: move to mswin_unsafe
            unsafe {
                let cs: &CREATESTRUCTA = &*(lparam.0 as *const CREATESTRUCTA);
                SetWindowLongPtrA(window, GWLP_USERDATA, cs.lpCreateParams as _);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            post_quit_message(0);
            LRESULT(0)
        }
        _ => {
            // todo: move to mswin_unsafe
            let ud = unsafe { GetWindowLongPtrA(window, GWLP_USERDATA) };
            let data = std::ptr::NonNull::new(ud as _);
            let handled = data.is_some_and(|mut d| handle_message_if_applicable(unsafe { d.as_mut() }, window, message, wparam, lparam));

            if handled {
                LRESULT::default()
            } else {
                default_window_proc(window, message, wparam, lparam)
            }
        }
    }
}

fn handle_message_if_applicable(input: &mut InputState, _window: HWND, message: u32, wparam: WPARAM, _lparam: LPARAM) -> bool {
    match message {
        WM_KEYDOWN => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_ESCAPE => {
                    post_quit_message(0);
                    true
                }
                VK_G => {
                    input.g_key = KeyDown(Instant::now());
                    true
                }
                _ => true
            }
        }
        WM_KEYUP => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_G => {
                    input.g_key = KeyUp();
                    true
                }
                _ => true
            }
        }
        _ => false
    }
}
