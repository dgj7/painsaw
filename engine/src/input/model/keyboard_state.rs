use std::time::{Duration, Instant};

#[derive(Clone,Debug)]
pub struct KeyInfo {
    pub when: Instant,
    pub handled: bool,
}

#[derive(Clone,Debug)]
pub enum KeyPosition {
    KeyDown { info: KeyInfo },
    KeyUp { info: KeyInfo },
}

#[derive(Clone,Debug)]
pub struct KeyState {
    pub previous: KeyPosition,
    pub current: KeyPosition,
}

impl KeyPosition {
    pub fn is_down(&self) -> bool {
        match *self {
            KeyPosition::KeyDown { .. } => true,
            KeyPosition::KeyUp { .. } => false,
        }
    }

    pub fn is_up(&self) -> bool {
        match *self {
            KeyPosition::KeyDown { .. } => false,
            KeyPosition::KeyUp { .. } => true,
        }
    }

    pub fn get_key_info(&self) -> KeyInfo {
        match self {
            KeyPosition::KeyDown { info } => info.clone(),
            KeyPosition::KeyUp { info } => info.clone(),
        }
    }
}

impl KeyState {
    pub fn update(&mut self, change: KeyPosition) {
        if (self.current.is_up() && change.is_up()) || (self.current.is_down() && change.is_down()) {
            return;
        } else {
            self.previous = self.current.clone();
            self.current = change;
        }
    }

    pub fn previous_key_state_duration(&self) -> Duration {
        self.current.get_key_info().when - self.previous.get_key_info().when
    }
}
