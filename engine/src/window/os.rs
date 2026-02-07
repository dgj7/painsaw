use crate::config::EngineConfig;
use crate::window::wc::WorldController;

pub mod mswin;

pub trait Window {
    fn begin_event_handling(&mut self, renderer: Box<dyn WorldController>, config: EngineConfig) -> Result<(), Box<dyn std::error::Error>>;
}

