use crate::config::EngineConfig;
use crate::WorldController;

pub mod os;
pub mod window_error;
pub mod api;

pub trait Window {
    fn begin_event_handling(&mut self, renderer: Box<dyn WorldController>, config: EngineConfig) -> Result<(), Box<dyn std::error::Error>>;
}
