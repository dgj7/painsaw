use crate::input::ii::InputInfo;
use crate::input::ic::InputChange;
use std::time::Duration;

#[derive(Clone,Debug)]
pub struct InputState {
    pub previous: InputChange,
    pub current: InputChange,
}

impl InputState {
    pub fn new(position: InputChange) -> InputState {
        InputState {
            previous: InputChange::Inactive { info: InputInfo::handled() },
            current: position,
        }
    }

    pub fn update(&mut self, change: InputChange) {
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
