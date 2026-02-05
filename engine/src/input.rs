use crate::graphics::geometry::dim::Dimension2D;
use crate::input::r#in::InputName;
use crate::input::ic::InputChange;
use crate::input::is::InputState;
use num_traits::Float;
use std::collections::{HashMap, VecDeque};
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};

pub mod is;
pub mod ii;
pub mod ic;
pub mod r#in;

#[derive(Clone,Debug)]
pub struct UserInput<F: Float + Add<F> + Sub<F>> {
    /* keyboard */
    pub changes: VecDeque<InputName>,
    pub states: HashMap<InputName, InputState>,

    /* screen */
    pub previous_client_dimensions: Dimension2D<F>,
    pub current_client_dimensions: Dimension2D<F>,
    pub previous_window_dimensions: Dimension2D<F>,
    pub current_window_dimensions: Dimension2D<F>,
    pub screen_resized: bool,
}

impl<F: Float + Add<F> + Sub<F>> UserInput<F> {
    pub fn new() -> Arc<Mutex<UserInput<F>>> {
        Arc::new(Mutex::new(UserInput {
            /* keyboard */
            changes: VecDeque::new(),
            states: HashMap::new(),

            /* screen */
            previous_client_dimensions: Dimension2D::new(F::zero(), F::zero()),
            current_client_dimensions: Dimension2D::new(F::zero(), F::zero()),
            previous_window_dimensions: Dimension2D::new(F::zero(), F::zero()),
            current_window_dimensions: Dimension2D::new(F::zero(), F::zero()),
            screen_resized: false,
        }))
    }

    pub fn handle_change(&mut self, name: InputName, position: InputChange) {
        self.changes.push_back(name.clone());
        self.states
            .entry(name)
            .and_modify(|e| e.update(position.clone()))
            .or_insert(InputState::new(position));
    }

    pub fn update_client_dimensions(&mut self, current: Dimension2D<F>) {
        /* copy existing current into previous */
        self.previous_client_dimensions.height = self.current_client_dimensions.height;
        self.previous_client_dimensions.width = self.current_client_dimensions.width;

        /* new info goes into current */
        self.current_client_dimensions.height = current.height;
        self.current_client_dimensions.width = current.width;
    }

    pub fn update_window_dimensions(&mut self, current: Dimension2D<F>) {
        /* copy existing current into previous */
        self.previous_window_dimensions.height = self.current_window_dimensions.height;
        self.previous_window_dimensions.width = self.current_window_dimensions.width;

        /* new info goes into current */
        self.current_window_dimensions.height = current.height;
        self.current_window_dimensions.width = current.width;
    }
}
