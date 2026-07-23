use crate::input::keyboard::kc::KeyChange;
use crate::input::keyboard::kii::KeyInputInfo;
use crate::input::keyboard::kin::KeyInputName;
use crate::input::mouse::mfs::MouseFunctionStatus;
use crate::input::mouse::min::MouseInputName;
use crate::input::UserInput;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::window::os::mswin::userdata::{create_and_write_pointer, read_window_data};
use crate::window::os::mswin::util::{get_client_rect_dim2d, get_window_rect_dim2d, is_mouse_over_window};
use crate::window::os::mswin::winapi::{default_window_proc, post_quit_message};
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_A, VK_D, VK_ESCAPE, VK_G, VK_M, VK_S, VK_W};
use windows::Win32::UI::Input::{GetRawInputData, HRAWINPUT, RAWINPUT, RAWINPUTHEADER, RID_INPUT, RIM_TYPEMOUSE};
use windows::Win32::UI::WindowsAndMessaging::{WM_CLOSE, WM_CREATE, WM_DESTROY, WM_INPUT, WM_KEYDOWN, WM_KEYUP, WM_KILLFOCUS, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_QUIT, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SETFOCUS, WM_SIZE};

///
/// required window procedure, for handling win32 event messages.
///
pub(crate) extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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
/// make it more obvious if a window message was handled.
///
static HANDLED: bool = true;
static NOT_HANDLED: bool = false;

///
/// handle input messages, such as key down/up or mouse movement.
///
fn handle_message_if_applicable(input: &Arc<Mutex<UserInput>>, hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> bool {
    match message {
        WM_KEYDOWN => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_ESCAPE => { input.lock().expect("todo: esc: down").record_keyboard_change(KeyInputName::KeyEscape, KeyChange::Active { info: KeyInputInfo::unhandled()});true }
                VK_A => { input.lock().expect("todo: a: down").record_keyboard_change(KeyInputName::KeyA, KeyChange::Active { info: KeyInputInfo::unhandled() }); true }
                VK_D => { input.lock().expect("todo: d: down").record_keyboard_change(KeyInputName::KeyD, KeyChange::Active { info: KeyInputInfo::unhandled() }); true }
                VK_G => { input.lock().expect("todo: g: down").record_keyboard_change(KeyInputName::KeyG, KeyChange::Active { info: KeyInputInfo::unhandled() }); true }
                VK_M => { input.lock().expect("todo: m: down").record_keyboard_change(KeyInputName::KeyM, KeyChange::Active { info: KeyInputInfo::unhandled() }); true }
                VK_S => { input.lock().expect("todo: s: down").record_keyboard_change(KeyInputName::KeyS, KeyChange::Active { info: KeyInputInfo::unhandled() }); true }
                VK_W => { input.lock().expect("todo: w: down").record_keyboard_change(KeyInputName::KeyW, KeyChange::Active { info: KeyInputInfo::unhandled() }); true }
                // todo: add remaining keys down
                _ => false
            }
        }
        WM_KEYUP => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_ESCAPE => { input.lock().expect("todo: esc: up").record_keyboard_change(KeyInputName::KeyEscape, KeyChange::Inactive { info: KeyInputInfo::unhandled() }); true }
                VK_A => { input.lock().expect("todo: a: up").record_keyboard_change(KeyInputName::KeyA, KeyChange::Inactive { info: KeyInputInfo::unhandled() }); true }
                VK_D => { input.lock().expect("todo: d: up").record_keyboard_change(KeyInputName::KeyD, KeyChange::Inactive { info: KeyInputInfo::unhandled() }); true }
                VK_G => { input.lock().expect("todo: g: up").record_keyboard_change(KeyInputName::KeyG, KeyChange::Inactive { info: KeyInputInfo::unhandled() }); true }
                VK_M => { input.lock().expect("todo: m: up").record_keyboard_change(KeyInputName::KeyM, KeyChange::Inactive { info: KeyInputInfo::unhandled() }); true }
                VK_S => { input.lock().expect("todo: s: up").record_keyboard_change(KeyInputName::KeyS, KeyChange::Inactive { info: KeyInputInfo::unhandled() }); true }
                VK_W => { input.lock().expect("todo: w: up").record_keyboard_change(KeyInputName::KeyW, KeyChange::Inactive { info: KeyInputInfo::unhandled() }); true }
                // todo: add remaining keys up
                _ => false
            }
        }
        WM_INPUT => {
            // see also: WM_MOUSEMOVE: lower precision mouse detection; can use get_x_lparam() and get_y_lparam() like other mouse functions do
            if let Some((x,y)) = gather_raw_mouse(hwnd, lparam) {
                input.lock().expect("todo: wm_input").record_mouse_change(MouseInputName::MouseMove {x, y}, x, y, &MouseFunctionStatus::Active);
                return HANDLED
            }
            NOT_HANDLED
        }
        WM_LBUTTONDOWN => {
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            input.lock().expect("todo: wm_mousemove").record_mouse_change(MouseInputName::MouseLeftButton {x, y}, x, y, &MouseFunctionStatus::Active);
            true
        }
        WM_LBUTTONUP => {
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            input.lock().expect("todo: wm_mousemove").record_mouse_change(MouseInputName::MouseLeftButton {x, y}, x, y, &MouseFunctionStatus::Inactive);
            true
        }
        WM_RBUTTONDOWN => {
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            input.lock().expect("todo: wm_mousemove").record_mouse_change(MouseInputName::MouseRightButton {x, y}, x, y, &MouseFunctionStatus::Active);
            true
        }
        WM_RBUTTONUP => {
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            input.lock().expect("todo: wm_mousemove").record_mouse_change(MouseInputName::MouseRightButton {x, y}, x, y, &MouseFunctionStatus::Inactive);
            true
        }
        WM_SIZE => {
            // see also: WM_SIZING: while the user is actively resizing the window
            // see also: WM_ENTERSIZEMOVE: resizing started
            // see also: WM_EXITSIZEMOVE: resizing ended
            let window_dimensions = get_window_rect_dim2d(hwnd);
            let client_dimensions = get_client_rect_dim2d(hwnd);
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
            input.lock().expect("todo: set-focus").focus.update(KeyChange::Active { info: KeyInputInfo::unhandled() });
            true
        }
        WM_KILLFOCUS => {
            input.lock().expect("todo: kill-focus").focus.update(KeyChange::Inactive { info: KeyInputInfo::unhandled() });
            true
        }
        // todo: add mouse scroll
        _ => false
    }
}

///
/// get the x coordinate of the mouse from lparam.
///
fn get_x_lparam(lparam: LPARAM) -> i32 {
    (lparam.0 & 0xffff) as i16 as i32
}

///
/// get the y coordinate of the mouse from lparam.
///
fn get_y_lparam(lparam: LPARAM) -> i32 {
    ((lparam.0 >> 16) & 0xffff) as i16 as i32
}

///
/// gather the mouse position as high-precision data.
///
// todo: move GetRawInputData to winapi.rs
fn gather_raw_mouse(hwnd: HWND, lparam: LPARAM) -> Option<(i32, i32)> {
    /* sc if not over our window */
    if !is_mouse_over_window(hwnd) {
        return None;
    }

    /* get sizeof RAWINPUT struct */
    let mut ds = 0;
    unsafe { GetRawInputData(HRAWINPUT(lparam.0 as *mut std::ffi::c_void), RID_INPUT, None, &mut ds, size_of::<RAWINPUTHEADER>() as u32,) };

    /* allocate buffer; retrieve data */
    let mut buffer = vec![0u8; ds as usize];
    let rs = unsafe { GetRawInputData(HRAWINPUT(lparam.0 as *mut std::ffi::c_void), RID_INPUT, Some(buffer.as_mut_ptr() as *mut std::ffi::c_void), &mut ds, size_of::<RAWINPUTHEADER>() as u32) };

    /* if data was received, return */
    if rs > 0 {
        let ri = unsafe { &*(buffer.as_ptr() as *const RAWINPUT) };
        if ri.header.dwType == RIM_TYPEMOUSE.0 {
            let md = unsafe { &ri.data.mouse };
            let dx = md.lLastX;
            let dy = md.lLastY;
            Some((dx, dy))
        } else {
            None
        }
    } else {
        None
    }
}
