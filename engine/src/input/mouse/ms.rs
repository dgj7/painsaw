use std::time::Instant;
use crate::input::mouse::mc::MouseChange;
use crate::input::mouse::mfs::MouseFunctionStatus;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

#[derive(Clone, Debug)]
pub struct MouseState {
    pub previous: MouseChange,
    pub current: MouseChange,
}

impl MouseState {
    pub fn new(x: i32, y: i32, status: MouseFunctionStatus) -> MouseState {
        MouseState {
            previous: MouseChange::handled(0, 0),
            current: MouseChange::unhandled(x, y, status),
        }
    }

    ///
    /// Update the mouse state with the given position and status.
    ///
    /// If the current mouse change has been handled, we can replace previous
    /// with current, and create a new value for current.
    ///
    /// If the current mouse change isn't handled, simply update the current
    /// value with the input parameters and leave previous unchanged.
    ///
    pub fn update(&mut self, x: i32, y: i32, status: &MouseFunctionStatus) {
        if self.current.handled {
            self.previous = self.current.clone();
            self.current = MouseChange::unhandled(x, y, status.clone());
            log(LogLevel::Info, &|| String::from(format!("new: x={}, y={}, status={:?}", x, y, status)));
        } else {
            self.current.x = x;
            self.current.y = y;
            self.current.status = status.clone();
            self.current.when = Instant::now();
            log(LogLevel::Info, &|| String::from(format!("update: x={}, y={}, status={:?}", x, y, status)));
        }
    }
}
