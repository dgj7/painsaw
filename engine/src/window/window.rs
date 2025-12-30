use crate::input::model::input_state::InputState;
use crate::window::wc::world_control::WorldController;
use std::sync::{Arc, Mutex};

pub trait Window {
    fn begin_event_handling(&mut self, renderer: &dyn WorldController<f32>) -> Result<(), Box<dyn std::error::Error>>;

    fn get_input_state(&self) -> Arc<Mutex<InputState<f32>>>;
}
