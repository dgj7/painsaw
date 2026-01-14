use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use num_traits::Float;
use crate::geometry::dim::Dimension2D;
use crate::input::ki::KeyInfo;
use crate::input::kp::KeyPosition;
use crate::input::ks::KeyState;

pub mod ks;
pub mod ki;
pub mod kp;

#[derive(Clone,Debug)]
pub struct InputState<F: Float + Add<F> + Sub<F>> {
    /* keyboard */
    pub g_key: KeyState,

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


            /* screen */
            previous_client_dimensions: Dimension2D::new(F::zero(), F::zero()),
            current_client_dimensions: Dimension2D::new(F::zero(), F::zero()),
            previous_window_dimensions: Dimension2D::new(F::zero(), F::zero()),
            current_window_dimensions: Dimension2D::new(F::zero(), F::zero()),
            screen_resized: false,
        }))
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
