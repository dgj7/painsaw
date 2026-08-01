use crate::input::mouse::mc::MouseChange;
use crate::input::mouse::mfs::MouseFunctionStatus;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct MouseState {
    pub previous: MouseChange,
    pub current: MouseChange,
}

impl MouseState {
    ///
    /// create a new mouse state.
    ///
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
        } else {
            self.current.x = x;
            self.current.y = y;
            self.current.status = status.clone();
            self.current.when = Instant::now();
        }
    }

    ///
    /// calculate the change in x, dx.
    ///
    pub fn change_x(state: &MouseState) -> f32 {
        (state.current.x - state.previous.x) as f32
    }

    ///
    /// calculate the change in y, dy.
    ///
    pub fn change_y(state: &MouseState) -> f32 {
        (state.current.y - state.previous.y) as f32
    }
}
