use logger::model::log_config::LoggerConfig;
use crate::render::renderer::Renderer;

pub trait Window {
    fn begin_event_handling(&mut self, renderer: &dyn Renderer, logger: &LoggerConfig) -> Result<(), Box<dyn std::error::Error>>;
}
