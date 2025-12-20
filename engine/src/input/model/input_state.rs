use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::input::model::keyboard_state::{KeyInfo, KeyPosition, KeyState};

#[derive(Clone,Debug)]
pub struct InputState {
    pub g_key: KeyState,
}

impl InputState {
    pub fn new() -> Arc<Mutex<InputState>> {
        Arc::new(Mutex::new(InputState {
            g_key: KeyState {
                previous: KeyPosition::KeyUp { info: KeyInfo { when: Instant::now(), handled: true } },
                current: KeyPosition::KeyUp { info: KeyInfo { when: Instant::now(), handled: true } }
            }
        }))
    }
}
