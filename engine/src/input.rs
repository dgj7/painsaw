use crate::graphics::geometry::dim::Dimension2D;
use crate::input::ki::KeyInfo;
use crate::input::kn::KeyName;
use crate::input::kp::KeyPosition;
use crate::input::ks::KeyState;
use num_traits::Float;
use std::collections::{HashMap, VecDeque};
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub mod ks;
pub mod ki;
pub mod kp;
pub mod kn;

#[derive(Clone,Debug)]
pub struct InputState<F: Float + Add<F> + Sub<F>> {
    /* keyboard */
    pub g_key: KeyState,

    pub changes: VecDeque<KeyName>,
    pub states: HashMap<KeyName,KeyState>,

    /* screen */
    pub previous_client_dimensions: Dimension2D<F>,
    pub current_client_dimensions: Dimension2D<F>,
    pub previous_window_dimensions: Dimension2D<F>,
    pub current_window_dimensions: Dimension2D<F>,
    pub screen_resized: bool,
}

impl<F: Float + Add<F> + Sub<F>> InputState<F> {
    pub fn new() -> Arc<Mutex<InputState<F>>> {
        Arc::new(Mutex::new(InputState {
            /* keyboard */
            g_key: KeyState {
                previous: KeyPosition::KeyUp { info: KeyInfo { when: Instant::now(), handled: true } },
                current: KeyPosition::KeyUp { info: KeyInfo { when: Instant::now(), handled: true } }
            },

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

    pub fn handle_change(&mut self, name: KeyName, position: KeyPosition) {
        self.changes.push_back(name.clone());
        self.states
            .entry(name)
            .and_modify(|e| e.update(position))
            .or_insert(KeyState::new());
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
