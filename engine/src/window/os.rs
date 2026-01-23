use std::sync::{Arc, Mutex};
use crate::input::InputState;
use crate::window::wc::WorldController;

pub mod mswin;

pub trait Window {
    fn begin_event_handling(&mut self, renderer: Box<dyn WorldController<f32>>) -> Result<(), Box<dyn std::error::Error>>;

    fn get_input_state(&self) -> Arc<Mutex<InputState<f32>>>;
}

