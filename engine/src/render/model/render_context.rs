use std::sync::{Arc, Mutex};
use crate::input::model::input_state::InputState;
use crate::logger::Logger;

pub struct RendererContext {
    pub first_frame_rendered: bool,
    pub frame_count: u128,

    pub input: Arc<Mutex<InputState>>,
    pub logger: Arc<Logger>,
}

impl RendererContext {
    pub(crate) fn new(logger: &Arc<Logger>, input: &Arc<Mutex<InputState>>) -> RendererContext {
        RendererContext {
            first_frame_rendered: false,
            frame_count: 0,

            input: input.clone(),
            logger: logger.clone(),
        }
    }
}
