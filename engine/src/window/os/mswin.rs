use crate::config::window_config::WindowDimensions;
use crate::config::EngineConfig;
use crate::graphics::subsystem::opengl::opengl_mswin::{init_opengl, swap_buffers};
use crate::graphics::subsystem::opengl::opengl_mswin_api::{get_dc, release_dc, wgl_delete_context, wgl_get_current_context, wgl_make_current};
use crate::graphics::subsystem::GraphicsSubSystem;
use crate::input::ii::InputInfo;
use crate::input::r#in::InputName;
use crate::input::ic::InputChange;
use crate::input::UserInput;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::window::context::RendererContext;
use crate::window::os::mswin::mswin_data::{create_and_write_pointer, input_state_to_raw_pointer, read_window_data};
use crate::window::os::mswin::mswin_winapi::{create_window_ex, default_window_proc, dispatch_message, get_client_rect, get_module_handle, get_window_rect, load_cursor, peek_message, post_quit_message, register_class, translate_message};
use crate::window::os::Window;
use crate::window::wc::WorldController;
use num_traits::Float;
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::OpenGL::HGLRC;
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_A, VK_D, VK_ESCAPE, VK_G, VK_M, VK_S, VK_W};
use windows::Win32::UI::WindowsAndMessaging::{CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, PM_REMOVE, WINDOW_EX_STYLE, WM_CREATE, WM_DESTROY, WM_KEYDOWN, WM_KEYUP, WM_QUIT, WM_CLOSE, WM_SIZE, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_THICKFRAME, WS_VISIBLE, WM_SETFOCUS, WM_KILLFOCUS};
use windows_core::{HSTRING, PCWSTR};

pub mod mswin_winapi;
pub mod mswin_data;
mod mswin_errors;

pub struct MsWinWindow {
    pub input: Arc<Mutex<UserInput<f32>>>,

    pub hinstance: HINSTANCE,
    pub wndclassw: WNDCLASSW,
    pub atom: u16,
    pub hwnd: HWND,
    pub quit: bool,

    pub grss: GraphicsSubSystem,

    pub hdc: HDC,
    pub hrc: HGLRC,
}

impl<F: Float> Window<F> for MsWinWindow {
    fn begin_event_handling(&mut self, wc: Box<dyn WorldController<F>>, config: EngineConfig<F>) -> Result<(), Box<dyn std::error::Error>>
    {
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
    pub fn new<F: Float>(request : &EngineConfig<F>) -> Result<Box<dyn Window<F>>, Box<dyn std::error::Error>> {
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

///
/// required window procedure, for handling win32 event messages.
///
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_CREATE => {// 0x0001: sent when createwindowex/createwindow is called; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create
            create_and_write_pointer(window, lparam);
            LRESULT(0)
        }
        WM_DESTROY => {// 0x0002: sent when (uncancellable) window removal (not shown anymore); https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy
            log(LogLevel::Debug, &|| String::from("WM_DESTROY"));

            post_quit_message(0);// this probably isn't necessary

            LRESULT(0)
        }
        WM_QUIT => {// 0x0012: called when PostQuitMessage(0) is called; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-quit
            log(LogLevel::Debug, &|| String::from("WM_QUIT"));

            let input = read_window_data(window).unwrap();
            drop(input);

            post_quit_message(0);// this probably isn't necessary

            LRESULT(0)
        }
        WM_CLOSE => {// 0x0010: called when window 'x' is clicked to close the window; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close
            log(LogLevel::Debug, &|| String::from("WM_CLOSE"));

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

///
/// handle input messages, such as key down/up or mouse movement.
///
fn handle_message_if_applicable(input: &Arc<Mutex<UserInput<f32>>>, hwnd: HWND, message: u32, wparam: WPARAM, _lparam: LPARAM) -> bool {
    match message {
        WM_KEYDOWN => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_ESCAPE => { input.lock().expect("todo: esc: down").handle_change(InputName::KeyEscape, InputChange::Active { info: InputInfo::unhandled()});true }
                VK_A => { input.lock().expect("todo: a: down").handle_change(InputName::KeyA, InputChange::Active { info: InputInfo::unhandled() }); true }
                VK_D => { input.lock().expect("todo: d: down").handle_change(InputName::KeyD, InputChange::Active { info: InputInfo::unhandled() }); true }
                VK_G => { input.lock().expect("todo: g: down").handle_change(InputName::KeyG, InputChange::Active { info: InputInfo::unhandled() }); true }
                VK_M => { input.lock().expect("todo: m: down").handle_change(InputName::KeyM, InputChange::Active { info: InputInfo::unhandled() }); true }
                VK_S => { input.lock().expect("todo: s: down").handle_change(InputName::KeyS, InputChange::Active { info: InputInfo::unhandled() }); true }
                VK_W => { input.lock().expect("todo: w: down").handle_change(InputName::KeyW, InputChange::Active { info: InputInfo::unhandled() }); true }
                // todo: add remaining keys down
                _ => false
            }
        }
        WM_KEYUP => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_ESCAPE => { input.lock().expect("todo: esc: up").handle_change(InputName::KeyEscape, InputChange::Inactive { info: InputInfo::unhandled() }); true }
                VK_A => { input.lock().expect("todo: a: up").handle_change(InputName::KeyA, InputChange::Inactive { info: InputInfo::unhandled() }); true }
                VK_D => { input.lock().expect("todo: d: up").handle_change(InputName::KeyD, InputChange::Inactive { info: InputInfo::unhandled() }); true }
                VK_G => { input.lock().expect("todo: g: up").handle_change(InputName::KeyG, InputChange::Inactive { info: InputInfo::unhandled() }); true }
                VK_M => { input.lock().expect("todo: m: up").handle_change(InputName::KeyM, InputChange::Inactive { info: InputInfo::unhandled() }); true }
                VK_S => { input.lock().expect("todo: s: up").handle_change(InputName::KeyS, InputChange::Inactive { info: InputInfo::unhandled() }); true }
                VK_W => { input.lock().expect("todo: w: up").handle_change(InputName::KeyW, InputChange::Inactive { info: InputInfo::unhandled() }); true }
                // todo: add remaining keys up
                _ => false
            }
        }
        WM_SIZE => {
            // see also: WM_SIZING: while the user is actively resizing the window
            // see also: WM_ENTERSIZEMOVE: resizing started
            // see also: WM_EXITSIZEMOVE: resizing ended
            let window_dimensions = get_window_rect(hwnd);
            let client_dimensions = get_client_rect(hwnd);
            log(LogLevel::Trace, &|| String::from(format!("window: {:?}", window_dimensions)));
            log(LogLevel::Trace, &|| String::from(format!("client: {:?}", client_dimensions)));
            match input.lock() {
                Ok(mut uin) => {
                    uin.screen_resized = true;
                    uin.update_client_dimensions(client_dimensions);
                    uin.update_window_dimensions(window_dimensions);
                }
                Err(_) => panic!("todo: wm_size")
            }
            true
        }
        WM_SETFOCUS => {
            input.lock().expect("todo: set-focus").focus.update(InputChange::Active { info: InputInfo::unhandled() });
            true
        }
        WM_KILLFOCUS => {
            input.lock().expect("todo: kill-focus").focus.update(InputChange::Inactive { info: InputInfo::unhandled() });
            true
        }
        _ => false
    }
}

///
/// cleanup opengl.
///
// todo: should this be removed?  perhaps to the renderer?  hard to say, needs research.
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
