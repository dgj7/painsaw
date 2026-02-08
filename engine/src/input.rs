use crate::graphics::geometry::dim::Dimension2D;
use crate::input::kin::KeyInputName;
use crate::input::ic::InputChange;
use crate::input::is::InputState;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use crate::input::ii::InputInfo;
use crate::input::min::MouseInputName;

pub mod is;
pub mod ii;
pub mod ic;
pub mod kin;
pub mod min;

#[derive(Clone,Debug)]
pub struct UserInput {
    /* keyboard */
    pub changes: VecDeque<KeyInputName>,
    pub states: HashMap<KeyInputName, InputState>,

    /* mouse */
    pub mouse_changes: VecDeque<MouseInputName>,

    /* screen */
    pub previous_client_dimensions: Dimension2D,
    pub current_client_dimensions: Dimension2D,
    pub previous_window_dimensions: Dimension2D,
    pub current_window_dimensions: Dimension2D,
    pub screen_resized: bool,
    pub focus: InputState,
}

impl UserInput {
    pub fn new() -> Arc<Mutex<UserInput>> {
        Arc::new(Mutex::new(UserInput {
            /* keyboard */
            changes: VecDeque::new(),
            states: HashMap::new(),

            /* mouse */
            mouse_changes: VecDeque::new(),

            /* screen */
            previous_client_dimensions: Dimension2D::new(0.0, 0.0),
            current_client_dimensions: Dimension2D::new(0.0, 0.0),
            previous_window_dimensions: Dimension2D::new(0.0, 0.0),
            current_window_dimensions: Dimension2D::new(0.0, 0.0),
            screen_resized: false,
            focus: InputState::new(InputChange::Active {info: InputInfo::handled()}),
        }))
    }

    pub fn handle_change(&mut self, name: KeyInputName, position: InputChange) {
        self.changes.push_back(name.clone());
        self.states
            .entry(name)
            .and_modify(|e| e.update(position.clone()))
            .or_insert(InputState::new(position));
    }

    pub fn move_mouse(&mut self, name: MouseInputName) {
        if let MouseInputName::MouseMove { .. } = name {
            self.mouse_changes.push_back(name);
        } else {
            panic!("expected InputName::MouseMove but got {}", name);
        }
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
