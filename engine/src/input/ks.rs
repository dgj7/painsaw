use crate::input::ki::KeyInfo;
use crate::input::kp::KeyPosition;
use std::time::Duration;

#[derive(Clone,Debug)]
pub struct KeyState {
    pub previous: KeyPosition,
    pub current: KeyPosition,
}

impl KeyState {
    pub fn new(position: KeyPosition) -> KeyState {
        KeyState {
            previous: KeyPosition::KeyUp { info: KeyInfo::handled() },
            current: position,
        }
    }

    pub fn update(&mut self, change: KeyPosition) {
        if (self.current.is_up() && change.is_up()) || (self.current.is_down() && change.is_down()) {
            return;
        } else {
            self.previous = self.current.clone();
            self.current = change;
        }
    }

    pub fn previous_key_state_duration(&self) -> Duration {
        self.current.clone_key_info().when - self.previous.clone_key_info().when
    }
}
