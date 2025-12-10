use crate::model::window::Window;
use crate::model::window_config::{WindowConfig, WindowDimensions};
use crate::mswin::mswin_unsafe::{create_window_ex, default_window_proc, dispatch_message, get_module_handle, load_cursor, peek_message, post_quit_message, register_class, translate_message, validate_rect};
use logger::model::log_config::LoggerConfig;
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_ESCAPE};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};
use crate::render::render_context::RendererContext;
use crate::render::renderer::Renderer;

pub struct MsWinWindow {
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
        let context = RendererContext::new();

        while !self.quit {
            if peek_message(&mut message, Default::default(), 0, 0, PM_REMOVE) {
                if message.message == WM_QUIT {
                    self.quit = true;
                    break;
                }

                let _ = translate_message(&message);
                dispatch_message(&message);
            } else {
                renderer.render_scene(&context);
            }
        }

        logger.info(&|| "after while(!quit)");
        Ok(())
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
            None,
            None,
            Option::from(hinstance),
            None,
        ).expect("CreateWindowEx* failed");

        /* done; returning handles to window so we can create device context later */
        Ok(Box::new(MsWinWindow {
            hinstance,
            wndclassa: wc,
            atom,
            hwnd,
            quit: false,
        }))
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_PAINT => {
            let _ = validate_rect(Option::from(window), None);
            LRESULT(0)
        }
        WM_DESTROY => {
            post_quit_message(0);
            LRESULT(0)
        }
        WM_KEYDOWN => {
            if (VIRTUAL_KEY(wparam.0 as u16)) == VK_ESCAPE {
                post_quit_message(0);
            }
            LRESULT(0)
        }
        _ => default_window_proc(window, message, wparam, lparam),
    }
}
