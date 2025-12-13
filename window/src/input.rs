use crate::input::keyboard_state::KeyState;

pub mod keyboard_state;

#[derive(Clone,Debug)]
pub struct InputState {
    pub g_key: KeyState,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            g_key: KeyState::KeyUp()
        }
    }
}

impl Default for InputState {
    fn default() -> InputState {
        InputState::new()
    }
}
