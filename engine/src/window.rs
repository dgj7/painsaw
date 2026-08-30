use crate::config::input_config::kc::KeyHandler;
use crate::config::input_config::mc::MouseHandler;
use crate::config::EngineConfig;
use crate::WorldController;

pub mod api;
pub mod error;
pub mod key;
pub mod mswin;

///
/// shared definition of a screen that we render to; not specific to any host operating system.
///
pub trait Window {
    fn begin_event_handling<T: KeyHandler + MouseHandler + WorldController + 'static>(
        &mut self,
        game: &T,
        config: EngineConfig,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
