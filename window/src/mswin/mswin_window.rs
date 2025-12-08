use crate::model::window::Window;
use crate::model::window_config::{WindowConfig, WindowDimensions};
use logger::model::log_config::LoggerConfig;
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_ESCAPE};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};

pub struct MsWinWindow {
    pub hinstance: HINSTANCE,
    pub wndclassa: WNDCLASSA,
    pub atom: u16,
    pub hwnd: HWND,
    pub quit: bool,
}

impl Window for MsWinWindow {
    fn begin_event_handling(&mut self, logger: &LoggerConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        logger.debug(&|| "begin event handling");
        unsafe {
            let mut message: MSG = MSG::default();

            while !self.quit {
                if PeekMessageA(&mut message, Default::default(), 0, 0, PM_REMOVE).into() {
                    if message.message == WM_QUIT {
                        self.quit = true;
                        break;
                    }

                    let _ = TranslateMessage(&message);
                    DispatchMessageA(&message);
                } else {
                    // todo: rendering code
                }
            }

            logger.info(&|| "after while(!quit)");
            Ok(())
        }
    }
}

impl MsWinWindow {
    pub fn new(request : &WindowConfig) -> std::result::Result<Box<dyn Window>, Box<dyn std::error::Error>> {
        unsafe {
            /* get handle instance */
            let hinstance: HINSTANCE = HINSTANCE::from(GetModuleHandleA(None)?);
            debug_assert!(hinstance.0 != std::ptr::null_mut());

            /* create the wnd class */
            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: hinstance,
                lpszClassName: s!("window"),
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            /* register the wnd class */
            let atom = RegisterClassA(&wc);
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
            let hwnd = CreateWindowExA(
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
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                let _ = ValidateRect(Option::from(window), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_KEYDOWN => {
                if (VIRTUAL_KEY(wparam.0 as u16)) == VK_ESCAPE {
                    PostQuitMessage(0);
                }
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
