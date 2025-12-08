use crate::model::window::Window;
use crate::model::window_config::WindowConfig;
use logger::model::log_config::LoggerConfig;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};

pub struct MsWinWindow {}

impl Window for MsWinWindow {
    fn begin_display(&self, logger: &LoggerConfig) -> std::result::Result<(), Box<dyn std::error::Error>> {
        logger.debug(&|| "begin window display");
        unsafe {
            let instance: HINSTANCE = HINSTANCE::from(GetModuleHandleA(None)?);
            debug_assert!(instance.0 != std::ptr::null_mut());

            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance,
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                window_class,
                s!("This is a sample window"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                Option::from(instance),
                None,
            ).expect("TODO: panic message");

            let mut message = MSG::default();

            while GetMessageA(&mut message, Option::from(HWND(std::ptr::null_mut())), 0, 0).into() {
                DispatchMessageA(&message);
            }

            Ok(())
        }
    }
}

impl MsWinWindow {
    pub fn new(_request : &WindowConfig) -> Box<dyn Window> {
        Box::new(MsWinWindow {})
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
