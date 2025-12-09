use windows::Win32::Foundation;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, PeekMessageA, TranslateMessage, MSG, PEEK_MESSAGE_REMOVE_TYPE};

pub(crate) fn peek_message(lpmsg: *mut MSG, hwnd: Option<Foundation::HWND>, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: PEEK_MESSAGE_REMOVE_TYPE) -> bool {
    unsafe { bool::from(PeekMessageA(lpmsg, hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg)) }
}

pub(crate) fn translate_message(lpmsg: *const MSG) -> bool {
    unsafe { bool::from(TranslateMessage(lpmsg)) }
}

pub(crate) fn dispatch_message(lpmsg: *const MSG) {
    unsafe { DispatchMessageA(lpmsg); }
}
