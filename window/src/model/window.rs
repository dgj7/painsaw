use logger::model::log_config::LoggerConfig;

pub trait Window {
    fn begin_event_handling(&mut self, logger: &LoggerConfig) -> Result<(), Box<dyn std::error::Error>>;
}
