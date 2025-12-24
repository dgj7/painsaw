use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::input::model::keyboard_state::{KeyInfo, KeyPosition, KeyState};
use crate::math::twod::dimension_2d::Dimension2D;

#[derive(Clone,Debug)]
pub struct InputState {
    /* keyboard */
    pub g_key: KeyState,

    /* screen */
    pub previous_client_dimensions: Dimension2D<f32>,
    pub current_client_dimensions: Dimension2D<f32>,
    pub previous_window_dimensions: Dimension2D<f32>,
    pub current_window_dimensions: Dimension2D<f32>,
}

impl InputState {
    pub fn new() -> Arc<Mutex<InputState>> {
        Arc::new(Mutex::new(InputState {
            /* keyboard */
            g_key: KeyState {
                previous: KeyPosition::KeyUp { info: KeyInfo { when: Instant::now(), handled: true } },
                current: KeyPosition::KeyUp { info: KeyInfo { when: Instant::now(), handled: true } }
            },


            /* screen */
            previous_client_dimensions: Dimension2D::new(0.0, 0.0),
            current_client_dimensions: Dimension2D::new(0.0, 0.0),
            previous_window_dimensions: Dimension2D::new(0.0, 0.0),
            current_window_dimensions: Dimension2D::new(0.0, 0.0),
        }))
    }

    pub fn update_client_dimensions(&mut self, current: Dimension2D<f32>) {
        /* copy existing current into previous */
        self.previous_client_dimensions.height = self.current_client_dimensions.height;
        self.previous_client_dimensions.width = self.current_client_dimensions.width;

        /* new info goes into current */
        self.current_client_dimensions.height = current.height;
        self.current_client_dimensions.width = current.width;
    }
    
    pub fn update_window_dimensions(&mut self, current: Dimension2D<f32>) {
        /* copy existing current into previous */
        self.previous_window_dimensions.height = self.current_window_dimensions.height;
        self.previous_window_dimensions.width = self.current_window_dimensions.width;

        /* new info goes into current */
        self.current_window_dimensions.height = current.height;
        self.current_window_dimensions.width = current.width;
    }
}
