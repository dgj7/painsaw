use crate::input::mouse::mfs::MouseFunctionStatus;
use crate::input::mouse::ms::MouseState;
use crate::input::screen::ScreenState;
use keyboard::kc::KeyChange;
use keyboard::kin::KeyInputName;
use keyboard::ks::KeyState;
use mouse::min::MouseInputName;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

pub mod keyboard;
pub mod mouse;
mod screen;

#[derive(Clone, Debug)]
pub struct UserInput {
    /* keyboard */
    pub key_changes: VecDeque<KeyInputName>,
    pub key_states: HashMap<KeyInputName, KeyState>,

    /* mouse */
    pub mouse_changes: VecDeque<MouseInputName>,
    pub mouse_states: HashMap<MouseInputName, MouseState>,

    /* screen */
    pub screen: ScreenState,
}

impl UserInput {
    pub fn new() -> Arc<Mutex<UserInput>> {
        Arc::new(Mutex::new(UserInput {
            /* keyboard */
            key_changes: VecDeque::new(),
            key_states: HashMap::new(),

            /* mouse */
            mouse_changes: VecDeque::new(),
            mouse_states:  HashMap::new(),

            /* screen */
            screen: ScreenState::new(),
        }))
    }

    pub fn record_keyboard_change(&mut self, name: KeyInputName, position: KeyChange) {
        self.key_changes.push_back(name.clone());
        self.key_states
            .entry(name)
            .and_modify(|e| e.update(position.clone()))
            .or_insert(KeyState::new(position));
    }

    pub fn record_mouse_change(&mut self, name: MouseInputName, x: i32, y: i32, status: &MouseFunctionStatus) {
        self.mouse_changes.push_back(name.clone());
        self.mouse_states
            .entry(name)
            .and_modify(|e| e.update(x, y, status))
            .or_insert(MouseState::new(x, y, status.clone()));
    }
}
