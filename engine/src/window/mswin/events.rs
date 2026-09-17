use crate::input::keyboard::kc::KeyChange;
use crate::input::keyboard::kii::KeyInputInfo;
use crate::input::keyboard::kin::KeyInputName;
use crate::input::mouse::md::MouseDelta;
use crate::input::mouse::mfs::MouseFunctionStatus;
use crate::input::mouse::min::MouseInputName;
use crate::input::UserInput;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::window::mswin::userdata::{create_and_write_pointer, read_window_data};
use crate::window::mswin::util::is_mouse_over_window;
use crate::window::mswin::winapi::{
    default_window_proc, get_cursor_pos, get_raw_input_data, post_quit_message, screen_to_client,
};
use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A, VK_B, VK_C, VK_D, VK_E, VK_ESCAPE, VK_F, VK_G, VK_H, VK_I, VK_J, VK_K, VK_L, VK_M, VK_N, VK_O, VK_OEM_3, VK_P, VK_Q, VK_R, VK_S, VK_T, VK_U, VK_V, VK_W, VK_X, VK_Y, VK_Z};
use windows::Win32::UI::Input::{HRAWINPUT, RAWINPUT, RAWINPUTHEADER, RID_INPUT, RIM_TYPEMOUSE};
use windows::Win32::UI::WindowsAndMessaging::{
    WM_CLOSE, WM_CREATE, WM_DESTROY, WM_INPUT, WM_KEYDOWN, WM_KEYUP, WM_KILLFOCUS, WM_LBUTTONDOWN,
    WM_LBUTTONUP, WM_QUIT, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SETFOCUS, WM_SIZE,
};

///
/// required window procedure, for handling win32 event messages.
///
pub(crate) extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_CREATE => {
            // 0x0001: sent when createwindowex/createwindow is called; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create
            create_and_write_pointer(window, lparam);
            LRESULT(0)
        }
        WM_DESTROY => {
            // 0x0002: sent when (uncancellable) window removal (not shown anymore); https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy
            log(LogLevel::Debug, &|| String::from("WM_DESTROY"));

            post_quit_message(0); // this probably isn't necessary

            LRESULT(0)
        }
        WM_QUIT => {
            // 0x0012: called when PostQuitMessage(0) is called; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-quit
            log(LogLevel::Debug, &|| String::from("WM_QUIT"));

            let input = read_window_data(window).unwrap();
            drop(input);

            post_quit_message(0); // this probably isn't necessary

            LRESULT(0)
        }
        WM_CLOSE => {
            // 0x0010: called when window 'x' is clicked to close the window; https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close
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
fn handle_message_if_applicable(
    input: &Arc<Mutex<UserInput>>,
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> bool {
    match message {
        WM_KEYDOWN => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_ESCAPE => { input.lock().expect("todo: esc: down").record_keyboard_change(KeyInputName::KeyEscape, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_OEM_3 => { input.lock().expect("todo: tilde: down").record_keyboard_change(KeyInputName::KeyTilde, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_A => { input.lock().expect("todo: a: down").record_keyboard_change(KeyInputName::KeyA, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_B => { input.lock().expect("todo: b: down").record_keyboard_change(KeyInputName::KeyB, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_C => { input.lock().expect("todo: c: down").record_keyboard_change(KeyInputName::KeyC, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_D => { input.lock().expect("todo: d: down").record_keyboard_change(KeyInputName::KeyD, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_E => { input.lock().expect("todo: e: down").record_keyboard_change(KeyInputName::KeyE, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_F => { input.lock().expect("todo: f: down").record_keyboard_change(KeyInputName::KeyF, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_G => { input.lock().expect("todo: g: down").record_keyboard_change(KeyInputName::KeyG, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_H => { input.lock().expect("todo: h: down").record_keyboard_change(KeyInputName::KeyH, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_I => { input.lock().expect("todo: i: down").record_keyboard_change(KeyInputName::KeyI, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_J => { input.lock().expect("todo: j: down").record_keyboard_change(KeyInputName::KeyJ, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_K => { input.lock().expect("todo: k: down").record_keyboard_change(KeyInputName::KeyK, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_L => { input.lock().expect("todo: l: down").record_keyboard_change(KeyInputName::KeyL, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_M => { input.lock().expect("todo: m: down").record_keyboard_change(KeyInputName::KeyM, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_N => { input.lock().expect("todo: n: down").record_keyboard_change(KeyInputName::KeyN, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_O => { input.lock().expect("todo: o: down").record_keyboard_change(KeyInputName::KeyO, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_P => { input.lock().expect("todo: p: down").record_keyboard_change(KeyInputName::KeyP, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_Q => { input.lock().expect("todo: q: down").record_keyboard_change(KeyInputName::KeyQ, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_R => { input.lock().expect("todo: r: down").record_keyboard_change(KeyInputName::KeyR, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_S => { input.lock().expect("todo: s: down").record_keyboard_change(KeyInputName::KeyS, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_T => { input.lock().expect("todo: t: down").record_keyboard_change(KeyInputName::KeyT, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_U => { input.lock().expect("todo: u: down").record_keyboard_change(KeyInputName::KeyU, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_V => { input.lock().expect("todo: v: down").record_keyboard_change(KeyInputName::KeyV, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_W => { input.lock().expect("todo: w: down").record_keyboard_change(KeyInputName::KeyW, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_X => { input.lock().expect("todo: x: down").record_keyboard_change(KeyInputName::KeyX, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_Y => { input.lock().expect("todo: y: down").record_keyboard_change(KeyInputName::KeyY, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_Z => { input.lock().expect("todo: z: down").record_keyboard_change(KeyInputName::KeyZ, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_1 => { input.lock().expect("todo: 1: down").record_keyboard_change(KeyInputName::Key1, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_2 => { input.lock().expect("todo: 2: down").record_keyboard_change(KeyInputName::Key2, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_3 => { input.lock().expect("todo: 3: down").record_keyboard_change(KeyInputName::Key3, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_4 => { input.lock().expect("todo: 4: down").record_keyboard_change(KeyInputName::Key4, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_5 => { input.lock().expect("todo: 5: down").record_keyboard_change(KeyInputName::Key5, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_6 => { input.lock().expect("todo: 6: down").record_keyboard_change(KeyInputName::Key6, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_7 => { input.lock().expect("todo: 7: down").record_keyboard_change(KeyInputName::Key7, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_8 => { input.lock().expect("todo: 8: down").record_keyboard_change(KeyInputName::Key8, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_9 => { input.lock().expect("todo: 9: down").record_keyboard_change(KeyInputName::Key9, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_0 => { input.lock().expect("todo: 0: down").record_keyboard_change(KeyInputName::Key0, KeyChange::Active { info: KeyInputInfo::unhandled(), }, );HANDLED }
                _ => NOT_HANDLED,
            }
        }
        WM_KEYUP => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_ESCAPE => { input.lock().expect("todo: esc: up").record_keyboard_change(KeyInputName::KeyEscape, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_OEM_3 => { input.lock().expect("todo: tilde: up").record_keyboard_change(KeyInputName::KeyTilde, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_A => { input.lock().expect("todo: a: up").record_keyboard_change(KeyInputName::KeyA, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_B => { input.lock().expect("todo: b: up").record_keyboard_change(KeyInputName::KeyB, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_C => { input.lock().expect("todo: c: up").record_keyboard_change(KeyInputName::KeyC, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_D => { input.lock().expect("todo: d: up").record_keyboard_change(KeyInputName::KeyD, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_E => { input.lock().expect("todo: e: up").record_keyboard_change(KeyInputName::KeyE, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_F => { input.lock().expect("todo: f: up").record_keyboard_change(KeyInputName::KeyF, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_G => { input.lock().expect("todo: g: up").record_keyboard_change(KeyInputName::KeyG, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_H => { input.lock().expect("todo: h: up").record_keyboard_change(KeyInputName::KeyH, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_I => { input.lock().expect("todo: i: up").record_keyboard_change(KeyInputName::KeyI, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_J => { input.lock().expect("todo: j: up").record_keyboard_change(KeyInputName::KeyJ, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_K => { input.lock().expect("todo: k: up").record_keyboard_change(KeyInputName::KeyK, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_L => { input.lock().expect("todo: l: up").record_keyboard_change(KeyInputName::KeyL, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_M => { input.lock().expect("todo: m: up").record_keyboard_change(KeyInputName::KeyM, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_N => { input.lock().expect("todo: n: up").record_keyboard_change(KeyInputName::KeyN, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_O => { input.lock().expect("todo: o: up").record_keyboard_change(KeyInputName::KeyO, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_P => { input.lock().expect("todo: p: up").record_keyboard_change(KeyInputName::KeyP, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_Q => { input.lock().expect("todo: q: up").record_keyboard_change(KeyInputName::KeyQ, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_R => { input.lock().expect("todo: r: up").record_keyboard_change(KeyInputName::KeyR, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_S => { input.lock().expect("todo: s: up").record_keyboard_change(KeyInputName::KeyS, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_T => { input.lock().expect("todo: t: up").record_keyboard_change(KeyInputName::KeyT, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_U => { input.lock().expect("todo: u: up").record_keyboard_change(KeyInputName::KeyU, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_V => { input.lock().expect("todo: v: up").record_keyboard_change(KeyInputName::KeyV, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_W => { input.lock().expect("todo: w: up").record_keyboard_change(KeyInputName::KeyW, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_X => { input.lock().expect("todo: x: up").record_keyboard_change(KeyInputName::KeyX, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_Y => { input.lock().expect("todo: y: up").record_keyboard_change(KeyInputName::KeyY, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_Z => { input.lock().expect("todo: z: up").record_keyboard_change(KeyInputName::KeyZ, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_1 => { input.lock().expect("todo: 1: up").record_keyboard_change(KeyInputName::Key1, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_2 => { input.lock().expect("todo: 2: up").record_keyboard_change(KeyInputName::Key2, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_3 => { input.lock().expect("todo: 3: up").record_keyboard_change(KeyInputName::Key3, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_4 => { input.lock().expect("todo: 4: up").record_keyboard_change(KeyInputName::Key4, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_5 => { input.lock().expect("todo: 5: up").record_keyboard_change(KeyInputName::Key5, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_6 => { input.lock().expect("todo: 6: up").record_keyboard_change(KeyInputName::Key6, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_7 => { input.lock().expect("todo: 7: up").record_keyboard_change(KeyInputName::Key7, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_8 => { input.lock().expect("todo: 8: up").record_keyboard_change(KeyInputName::Key8, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_9 => { input.lock().expect("todo: 9: up").record_keyboard_change(KeyInputName::Key9, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                VK_0 => { input.lock().expect("todo: 0: up").record_keyboard_change(KeyInputName::Key0, KeyChange::Inactive { info: KeyInputInfo::unhandled(), }, );HANDLED }
                _ => NOT_HANDLED,
            }
        }
        WM_INPUT => {
            // see also: WM_MOUSEMOVE: lower precision mouse detection; can use get_x_lparam() and get_y_lparam() like other mouse functions do
            if let Some(md) = gather_raw_mouse(hwnd, lparam) {
                if let Ok(mut uin) = input.try_lock() {
                    /* add a delta */
                    uin.mouse_deltas.push(md);

                    /* grab mouse position and send event */
                    let mut pos = get_cursor_pos();
                    screen_to_client(hwnd, &mut pos);
                    uin.record_mouse_change(MouseInputName::MouseMove, pos.x, pos.y, &MouseFunctionStatus::Active, );
                }
                return HANDLED;
            }
            NOT_HANDLED
        }
        WM_LBUTTONDOWN => {
            // todo: can we source these from wm_input instead?
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            input.lock()
                .expect("todo: wm_mousemove")
                .record_mouse_change(MouseInputName::MouseLeftButton, x, y, &MouseFunctionStatus::Active, );
            HANDLED
        }
        WM_LBUTTONUP => {
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            input.lock()
                .expect("todo: wm_mousemove")
                .record_mouse_change(MouseInputName::MouseLeftButton, x, y, &MouseFunctionStatus::Inactive, );
            HANDLED
        }
        WM_RBUTTONDOWN => {
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            input.lock()
                .expect("todo: wm_mousemove")
                .record_mouse_change(MouseInputName::MouseRightButton, x, y, &MouseFunctionStatus::Active, );
            HANDLED
        }
        WM_RBUTTONUP => {
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            input.lock()
                .expect("todo: wm_mousemove")
                .record_mouse_change(MouseInputName::MouseRightButton, x, y, &MouseFunctionStatus::Inactive, );
            HANDLED
        }
        WM_SIZE => {
            // see also: WM_SIZING: while the user is actively resizing the window
            // see also: WM_ENTERSIZEMOVE: resizing started
            // see also: WM_EXITSIZEMOVE: resizing ended
            input.lock().expect("todo: wm_size").screen_resized = true;
            HANDLED
        }
        WM_SETFOCUS => {
            input
                .lock()
                .expect("todo: set-focus")
                .focus
                .update(KeyChange::Active { info: KeyInputInfo::unhandled(), });
            HANDLED
        }
        WM_KILLFOCUS => {
            input
                .lock()
                .expect("todo: kill-focus")
                .focus
                .update(KeyChange::Inactive { info: KeyInputInfo::unhandled(), });
            HANDLED
        }
        // todo: add mouse scroll
        _ => NOT_HANDLED,
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
/// here we're hijacking the high refresh rate, but calling GetCursorPos() regardless
/// because dx/dy isn't what we're looking for.
///
fn gather_raw_mouse(hwnd: HWND, lparam: LPARAM) -> Option<MouseDelta> {
    /* sc if not over our window */
    if !is_mouse_over_window(hwnd) {
        return None;
    }

    /* get sizeof RAWINPUT struct */
    let mut ds = 0;
    get_raw_input_data(HRAWINPUT(lparam.0 as *mut std::ffi::c_void), RID_INPUT, None, &mut ds, size_of::<RAWINPUTHEADER>() as u32, );

    /* allocate buffer; retrieve data */
    let mut buffer = vec![0u8; ds as usize];
    let rs = get_raw_input_data(HRAWINPUT(lparam.0 as *mut std::ffi::c_void), RID_INPUT, Some(buffer.as_mut_ptr() as *mut std::ffi::c_void), &mut ds, size_of::<RAWINPUTHEADER>() as u32, );

    /* if data was received, return */
    if rs > 0 {
        let ri = unsafe { &*(buffer.as_ptr() as *const RAWINPUT) };
        if ri.header.dwType == RIM_TYPEMOUSE.0 {
            let md = unsafe { &ri.data.mouse };
            let usbf = unsafe { md.Anonymous.Anonymous.usButtonFlags }; //RI_MOUSE_LEFT_BUTTON_DOWN,RI_MOUSE_LEFT_BUTTON_UP,RI_MOUSE_RIGHT_BUTTON_DOWN,RI_MOUSE_RIGHT_BUTTON_UP
            if usbf > 0 {
                return None;
            }

            if md.lLastX == 0 && md.lLastY == 0 {
                None
            } else {
                Some(MouseDelta {
                    dx: md.lLastX as f32,
                    dy: md.lLastY as f32,
                })
            }
        } else {
            None
        }
    } else {
        None
    }
}
