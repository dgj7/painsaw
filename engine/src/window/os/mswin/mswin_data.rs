use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrA, SetWindowLongPtrA, CREATESTRUCTA, GWLP_USERDATA};
use crate::input::InputState;

///
/// InputState to raw pointer.
///
/// Wrap with Some(is) and pass to CreateWindowExA().  Call within window creation method.
///
pub(crate) fn input_state_to_raw_pointer(is: &Arc<Mutex<InputState<f32>>>) -> *const core::ffi::c_void {
    let rc = Arc::clone(is);
    Box::into_raw(Box::new(rc)) as *mut _
}

///
/// Write data to CREATESTRUCTA.
///
/// Call within WM_CREATE.
///
pub(crate) fn create_and_write_pointer(hwnd: HWND, lparam: LPARAM) {
    let cs: &CREATESTRUCTA = unsafe { &*(lparam.0 as *const CREATESTRUCTA) };
    let ptr = cs.lpCreateParams;
    unsafe { SetWindowLongPtrA(hwnd, GWLP_USERDATA, ptr as isize) };
}

///
/// Read the window data.
///
/// Call within event-handling procedure.
///
pub(crate) fn read_window_data(hwnd: HWND) -> Option<Arc<Mutex<InputState<f32>>>> {
    let ptr = unsafe { GetWindowLongPtrA(hwnd, GWLP_USERDATA) };
    if ptr == 0 {
        return None;
    }
    let x = unsafe { &*(ptr as *const Arc<Mutex<InputState<f32>>>) };
    Option::from(x.clone())
}


