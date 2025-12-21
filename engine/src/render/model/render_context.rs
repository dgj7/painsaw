use std::sync::{Arc, Mutex};
use crate::input::model::input_state::InputState;
use crate::math::twod::dimension_2d::Dimension2D;

pub struct RendererContext {
    pub first_frame_rendered: bool,
    pub frame_count: u128,

    pub input: Arc<Mutex<InputState>>,
}

impl RendererContext {
    pub(crate) fn new(input: &Arc<Mutex<InputState>>) -> RendererContext {
        RendererContext {
            first_frame_rendered: false,
            frame_count: 0,

            input: input.clone(),
        }
    }

    pub fn copy_client_dimensions(&self) -> Dimension2D {
        self.input
            .lock()
            .expect("retrieve_client_dimensions(): can't lock")
            .current_client_dimensions
            .clone()
    }
}
