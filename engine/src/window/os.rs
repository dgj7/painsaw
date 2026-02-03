use crate::config::EngineConfig;
use crate::window::wc::WorldController;
use num_traits::Float;

pub mod mswin;

pub trait Window<F: Float> {
    fn begin_event_handling(&mut self, renderer: Box<dyn WorldController<F>>, config: EngineConfig<F>) -> Result<(), Box<dyn std::error::Error>>;
}

