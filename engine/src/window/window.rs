use std::sync::{Arc, Mutex};
use crate::input::model::input_state::InputState;
use crate::render::renderer::Renderer;

pub trait Window {
    fn begin_event_handling(&mut self, renderer: &dyn Renderer) -> Result<(), Box<dyn std::error::Error>>;

    fn get_input_state(&self) -> Arc<Mutex<InputState>>;
}
