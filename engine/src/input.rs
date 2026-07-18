use crate::geometry::dim::Dimension2D;
use keyboard::kc::KeyChange;
use keyboard::kii::KeyInputInfo;
use keyboard::ks::KeyState;
use keyboard::kin::KeyInputName;
use mouse::min::MouseInputName;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use crate::input::mouse::mfs::MouseFunctionStatus;
use crate::input::mouse::ms::MouseState;

pub mod keyboard;
pub mod mouse;

#[derive(Clone, Debug)]
pub struct UserInput {
    /* keyboard */
    pub key_changes: VecDeque<KeyInputName>,
    pub key_states: HashMap<KeyInputName, KeyState>,

    /* mouse */
    pub mouse_changes: VecDeque<MouseInputName>,
    pub mouse_states: HashMap<MouseInputName, MouseState>,

    /* screen */
    pub previous_client_dimensions: Dimension2D,
    pub current_client_dimensions: Dimension2D,
    pub previous_window_dimensions: Dimension2D,
    pub current_window_dimensions: Dimension2D,
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
            previous_client_dimensions: Dimension2D::new(0.0, 0.0),
            current_client_dimensions: Dimension2D::new(0.0, 0.0),
            previous_window_dimensions: Dimension2D::new(0.0, 0.0),
            current_window_dimensions: Dimension2D::new(0.0, 0.0),
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
        self.mouse_changes.push_back(name.clone());
        self.mouse_states
            .entry(name)
            .and_modify(|e| e.update(x, y, status))
            .or_insert(MouseState::new(x, y, status.clone()));
    }

    pub fn update_client_dimensions(&mut self, current: Dimension2D) {
        /* copy existing current into previous */
        self.previous_client_dimensions.height = self.current_client_dimensions.height;
        self.previous_client_dimensions.width = self.current_client_dimensions.width;

        /* new info goes into current */
        self.current_client_dimensions.height = current.height;
        self.current_client_dimensions.width = current.width;
    }

    pub fn update_window_dimensions(&mut self, current: Dimension2D) {
        /* copy existing current into previous */
        self.previous_window_dimensions.height = self.current_window_dimensions.height;
        self.previous_window_dimensions.width = self.current_window_dimensions.width;

        /* new info goes into current */
        self.current_window_dimensions.height = current.height;
        self.current_window_dimensions.width = current.width;
    }
}
