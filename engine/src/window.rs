use std::sync::Arc;
use crate::config::EngineConfig;
use crate::WorldController;

pub mod error;
pub mod api;
pub mod mswin;
pub mod key;

///
/// shared definition of a screen that we render to; not specific to any host operating system.
/// 
pub trait Window {
    fn begin_event_handling(&mut self, renderer: Arc<dyn WorldController>, config: EngineConfig) -> Result<(), Box<dyn std::error::Error>>;
}
