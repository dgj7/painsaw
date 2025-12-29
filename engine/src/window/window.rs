use crate::input::model::input_state::InputState;
use crate::window::render::renderer::Renderer;
use std::sync::{Arc, Mutex};

pub trait Window {
    fn begin_event_handling(&mut self, renderer: &dyn Renderer<f32>) -> Result<(), Box<dyn std::error::Error>>;

    fn get_input_state(&self) -> Arc<Mutex<InputState<f32>>>;
}
