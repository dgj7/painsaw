use std::sync::{Arc, Mutex};
use num_traits::Float;
use crate::config::EngineConfig;
use crate::input::InputState;
use crate::window::wc::WorldController;

pub mod mswin;

pub trait Window<F: Float> {
    fn begin_event_handling(&mut self, renderer: Box<dyn WorldController<F>>, config: EngineConfig<F>) -> Result<(), Box<dyn std::error::Error>>;

    fn get_input_state(&self) -> Arc<Mutex<InputState<f32>>>;
}

