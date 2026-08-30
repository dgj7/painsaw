use crate::config::input_config::kc::KeyHandler;
use crate::config::input_config::mc::MouseHandler;
use crate::config::window_config::WindowDimensions;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::subsystem::opengl::msw::window::{init_opengl, opengl_cleanup, swap_buffers};
use crate::graphics::subsystem::GraphicsSubSystem;
use crate::graphics::GraphicsIntermediary;
use crate::input::screen::ScreenState;
use crate::input::UserInput;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::support::timing::EngineTiming;
use crate::window::key::WindowKey;
use crate::window::mswin::events::wndproc;
use crate::window::mswin::userdata::input_state_to_raw_pointer;
use crate::window::mswin::winapi::{
    create_window_ex, dispatch_message, get_module_handle, load_cursor, peek_message,
    register_class, register_raw_input_devices, translate_message,
};
use crate::window::Window;
use crate::WorldController;
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::UI::Input::{RAWINPUTDEVICE, RAWINPUTDEVICE_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::{
    CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, PM_REMOVE, WINDOW_EX_STYLE,
    WM_QUIT, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_THICKFRAME, WS_VISIBLE,
};
use windows_core::{HSTRING, PCWSTR};

pub mod errors;
pub mod events;
pub mod userdata;
pub mod util;
pub mod winapi;

pub struct MsWinWindow {
    pub input: Arc<Mutex<UserInput>>,
    pub quit: bool,

    pub grss: GraphicsSubSystem,

    pub key: WindowKey,
}

impl Window for MsWinWindow {
    fn begin_event_handling<T: KeyHandler + MouseHandler + WorldController + 'static>(
        &mut self,
        config: EngineConfig,
        game: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log(LogLevel::Info, &|| "begin event handling".parse().unwrap());
        let mut message: MSG = MSG::default();
        let mut screen = ScreenState::from(&self.key);
        let mut timing = EngineTiming::new(&config.renderer);
        let mut g2d = Graph2D::new();
        let mut g3d = Graph3D::new();
        let mut camera = Camera::new(&screen.current_client_dimensions);
        let mut graphics = GraphicsIntermediary::new(config.renderer.graphics.clone());
        

        /* initialize client renderer, if necessary */
        game.initialize_world(&mut graphics, &camera, &mut g2d, &mut g3d, );

        while !self.quit {
            if peek_message(&mut message, Default::default(), 0, 0, PM_REMOVE) {
                if message.message == WM_QUIT {
                    log(LogLevel::Debug, &|| String::from("WM_QUIT"));
                    self.quit = true;
                    opengl_cleanup(self.key.hwnd);
                    break;
                }

                let _ = translate_message(&message);
                dispatch_message(&message);
            } else if timing.is_ok_to_render() {
                /* timing */
                timing.begin_frame();

                /* update world info; graphics scene */
                game.update_world(game, self.input.clone(), &mut camera, &config, &mut screen, &mut timing, &graphics, &mut g2d, &mut g3d, &self.key, );
                game.display_world_scene(game, self.input.clone(), &mut camera, &config, &mut screen, &timing, &mut graphics, &mut g2d, &mut g3d, );

                /* swap buffers after it's all done */
                swap_buffers(self.key.hdc);

                /* timing */
                timing.end_frame();
            }
        }

        log(LogLevel::Info, &|| { return String::from(format!("after while(!quit); rendered {} frames", timing.frame_count)); });

        Ok(())
    }
}

impl MsWinWindow {
    ///
    /// create a new instance.
    ///
    pub(crate) fn new(request: &EngineConfig) -> Result<Box<Self>, Box<dyn std::error::Error>> {
        /* make some variables */
        let wndclass = PCWSTR::from_raw(HSTRING::from(request.window.window_id.clone().unwrap_or(String::from("WindowConfig: set wndclass"))).as_ptr(), );
        let title = PCWSTR::from_raw(HSTRING::from(request.window.title.clone().unwrap_or(String::from("WindowConfig: set title")), ).as_ptr(), );
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
            WindowDimensions::Dimensional { width: _width, height: _height, } => WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_THICKFRAME,
        };
        let (x, y) = match request.window.dimensions {
            WindowDimensions::Fullscreen => (0, 0),
            WindowDimensions::Dimensional { width: _width, height: _height, } => (CW_USEDEFAULT, CW_USEDEFAULT),
        };
        let (width, height) = match request.window.dimensions {
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
            None, // no parent window
            None, // no menus
            Option::from(hinstance),
            Some(input_pointer),
        )
        .expect("CreateWindowEx* failed");

        /* register raw input for mouse movement detection */
        init_raw(&hwnd);

        /* init opengl */
        let (hdc, hrc) = init_opengl(hwnd);

        /* done; returning handles to window */
        Ok(Box::new(MsWinWindow {
            input,
            quit: false,

            grss,

            key: WindowKey {
                hinstance,
                wndclassw: wc,
                atom,
                hwnd,
                hdc,
                hrc,
            },
        }))
    }
}

///
/// initialize WM_INPUT raw input device.  in this case, we're using it for mouse movement.
///
/// normal keyboard input tracking for win32 works well enough, so that won't be configured here.
///
fn init_raw(hwnd: &HWND) {
    /* create array of RIDs */
    let rid = [/* create RID for mouse */ RAWINPUTDEVICE {
        usUsagePage: 0x01,                // generic desktop
        usUsage: 0x02,                    // mouse=0x02, keyboard=0x06
        dwFlags: RAWINPUTDEVICE_FLAGS(0), // RIDEV_INPUTSINK: recv input even when in background (not in focus)
        hwndTarget: *hwnd,
    }];

    register_raw_input_devices(&rid);
}
