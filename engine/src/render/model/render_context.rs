use std::sync::{Arc, Mutex};
use crate::input::model::input_state::InputState;

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
}
