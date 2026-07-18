use std::time::Instant;
use crate::input::mouse::mfs::MouseFunctionStatus;

#[derive(Clone, Debug)]
pub struct MouseChange {
    pub x: i32,
    pub y: i32,
    pub when: Instant,
    pub handled: bool,
    pub status: MouseFunctionStatus,
}

impl MouseChange {
    pub fn unhandled(x: i32, y: i32, status: MouseFunctionStatus) -> MouseChange {
        MouseChange {
            x,
            y,
            when: Instant::now(),
            handled: false,
            status,
        }
    }

    pub fn handled(x: i32, y: i32) -> MouseChange {
        MouseChange {
            x,
            y,
            when: Instant::now(),
            handled: true,
            status: MouseFunctionStatus::Inactive,
        }
    }
}
