use crate::input::keyboard::kii::KeyInputInfo;
use crate::input::keyboard::kc::KeyChange;
use std::time::Duration;

#[derive(Clone,Debug)]
pub struct KeyState {
    pub previous: KeyChange,
    pub current: KeyChange,
}

impl KeyState {
    pub fn new(position: KeyChange) -> KeyState {
        KeyState {
            previous: KeyChange::Inactive { info: KeyInputInfo::handled() },
            current: position,
        }
    }

    pub fn update(&mut self, change: KeyChange) {
        if (self.current.is_inactive() && change.is_inactive()) || (self.current.is_active() && change.is_active()) {
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
