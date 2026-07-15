use crate::input::mc::MouseChange;

pub struct MouseState {
    pub previous: MouseChange,
    pub current: MouseChange,
}
