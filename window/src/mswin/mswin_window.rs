use crate::model::window::Window;
use crate::model::window_config::{WindowConfig, WindowDimensions};
use logger::model::log_config::LoggerConfig;
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
}

impl Window for MsWinWindow {
    fn begin_event_handling(&self, logger: &LoggerConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        logger.debug(&|| "begin event handling");
        unsafe {
            let mut message = MSG::default();

            while GetMessageA(&mut message, Option::from(HWND(std::ptr::null_mut())), 0, 0).into() {
                DispatchMessageA(&message);
            }

            Ok(())
        }
    }
}

impl MsWinWindow {
    pub fn new(request : &WindowConfig) -> std::result::Result<Box<dyn Window>, Box<dyn std::error::Error>> {
        unsafe {
            let hinstance: HINSTANCE = HINSTANCE::from(GetModuleHandleA(None)?);
            debug_assert!(hinstance.0 != std::ptr::null_mut());

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: hinstance,
                lpszClassName: s!("window"),
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let (width,height) = match request.dimensions {
                WindowDimensions::Fullscreen => (CW_USEDEFAULT, CW_USEDEFAULT),
                WindowDimensions::Dimensional { width, height } => (width, height),
            };

            let hwnd = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                s!("window"),
                s!("This is a sample window"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width,
                height,
                None,
                None,
                Option::from(hinstance),
                None,
            ).expect("CreateWindowEx* failed");

            Ok(Box::new(MsWinWindow {
                hinstance,
                wndclassa: wc,
                atom,
                hwnd,
            }))
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                let _ = ValidateRect(Option::from(window), None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
