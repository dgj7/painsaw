use crate::config::window_config::WindowDimensions;
use crate::config::EngineConfig;
use crate::graphics::subsystem::opengl::msw::window::{init_opengl, opengl_cleanup, swap_buffers};
use crate::graphics::subsystem::GraphicsSubSystem;
use crate::input::UserInput;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::window::context::RendererContext;
use crate::window::os::mswin::events::wndproc;
use crate::window::os::mswin::userdata::input_state_to_raw_pointer;
use crate::window::os::mswin::winapi::{create_window_ex, dispatch_message, get_module_handle, load_cursor, peek_message, register_class, translate_message};
use crate::window::os::Window;
use crate::window::wc::WorldController;
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::OpenGL::HGLRC;
use windows::Win32::UI::WindowsAndMessaging::{CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, PM_REMOVE, WINDOW_EX_STYLE, WM_QUIT, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_THICKFRAME, WS_VISIBLE};
use windows_core::{HSTRING, PCWSTR};

pub mod winapi;
pub mod userdata;
pub mod errors;
pub mod events;

pub struct MsWinWindow {
    pub input: Arc<Mutex<UserInput>>,

    pub hinstance: HINSTANCE,
    pub wndclassw: WNDCLASSW,
    pub atom: u16,
    pub hwnd: HWND,
    pub quit: bool,

    pub grss: GraphicsSubSystem,

    pub hdc: HDC,
    pub hrc: HGLRC,
}

impl Window for MsWinWindow {
    fn begin_event_handling(&mut self, wc: Box<dyn WorldController>, config: EngineConfig) -> Result<(), Box<dyn std::error::Error>> {
        log(LogLevel::Info, &|| "begin event handling".parse().unwrap());
        let mut message: MSG = MSG::default();
        let mut context = RendererContext::new(&self.input, config);

        /* initialize client renderer, if necessary */
        wc.initialize_world(&mut context);

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
            } else if context.timing.is_ok_to_render() {
                /* timing */
                context.timing.begin_frame();

                /* update world info; graphics scene */
                wc.update_world(&mut context);
                wc.display_world_scene(&mut context);

                /* swap buffers after it's all done */
                swap_buffers(self.hdc);

                /* timing */
                context.timing.end_frame();
            }
        }

        log(LogLevel::Info, &|| { return String::from(format!("after while(!quit); rendered {} frames", context.frame_count)) });

        Ok(())
    }
}

impl MsWinWindow {
    ///
    /// create a new instance.
    ///
    pub fn new(request : &EngineConfig) -> Result<Box<dyn Window>, Box<dyn std::error::Error>> {
        /* make some variables */
        let wndclass = PCWSTR::from_raw(HSTRING::from(request.window.window_id.clone().unwrap_or(String::from("WindowConfig: set wndclass"))).as_ptr());
        let title = PCWSTR::from_raw(HSTRING::from(request.window.title.clone().unwrap_or(String::from("WindowConfig: set title"))).as_ptr());
        let grss = request.renderer.graphics.clone();

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
        let dwstyle = match request.window.dimensions {
            WindowDimensions::Fullscreen => WS_VISIBLE,
            WindowDimensions::Dimensional { width: _width, height: _height } => WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_THICKFRAME,
        };
        let (x, y) = match request.window.dimensions {
            WindowDimensions::Fullscreen => (0, 0),
            WindowDimensions::Dimensional { width: _width, height: _height } => (CW_USEDEFAULT, CW_USEDEFAULT),
        };
        let (width,height) = match request.window.dimensions {
            WindowDimensions::Fullscreen => (CW_USEDEFAULT, CW_USEDEFAULT),
            WindowDimensions::Dimensional { width, height } => (width, height),
        };

        /* create input state */
        let input = UserInput::new();
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

            grss,

            hdc,
            hrc,
        }))
    }
}
