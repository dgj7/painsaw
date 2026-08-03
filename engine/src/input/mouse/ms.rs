use crate::input::mouse::mc::MouseChange;
use crate::input::mouse::mfs::MouseFunctionStatus;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct MouseState {
    pub previous: Option<MouseChange>,
    pub current: MouseChange,
}

impl MouseState {
    ///
    /// create a new mouse state.
    ///
    pub fn new(x: i32, y: i32, status: MouseFunctionStatus) -> MouseState {
        MouseState {
            previous: None,
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
            self.previous = Some(self.current.clone());
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
    pub fn change_x(state: &MouseState) -> Option<f32> {
        if let Some(previous) = &state.previous {
            return Some(state.current.x as f32 - previous.x as f32);
        }
        None
    }

    ///
    /// calculate the change in y, dy.
    ///
    pub fn change_y(state: &MouseState) -> Option<f32> {
        if let Some(previous) = &state.previous {
            return Some(state.current.y as f32 - previous.y as f32);
        }
        None
    }
}
