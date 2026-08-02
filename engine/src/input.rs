use crate::input::keyboard::kii::KeyInputInfo;
use crate::input::mouse::mfs::MouseFunctionStatus;
use crate::input::mouse::ms::MouseState;
use keyboard::kc::KeyChange;
use keyboard::kin::KeyInputName;
use keyboard::ks::KeyState;
use mouse::min::MouseInputName;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub mod keyboard;
pub mod mouse;
pub mod screen;

#[derive(Clone, Debug)]
pub struct UserInput {
    /* keyboard */
    pub key_changes: VecDeque<KeyInputName>,
    pub key_states: HashMap<KeyInputName, KeyState>,

    /* mouse */
    pub mouse_changes: VecDeque<MouseInputName>,
    pub mouse_states: HashMap<MouseInputName, MouseState>,

    /* screen */
    pub screen_resized: bool,
    pub focus: KeyState,
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
            screen_resized: false,
            focus: KeyState::new(KeyChange::Active {
                info: KeyInputInfo::handled(),
            }),
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
        /* don't make any update if the current position is equal to the update */
        if let Some(pos) = self.mouse_states.get(&name) {
            if pos.current.x == x && pos.current.y == y {
                return;
            }
        }

        log(LogLevel::Info, &|| format!("MouseMove({},{})", x, y));

        /* update */
        self.mouse_changes.push_back(name.clone());
        self.mouse_states
            .entry(name)
            .and_modify(|e| e.update(x, y, status))
            .or_insert(MouseState::new(x, y, status.clone()));
    }
}
