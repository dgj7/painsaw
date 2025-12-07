use logger::model::log_config::LoggerConfig;

pub trait Window {
    fn begin_display(&self, logger: &LoggerConfig);
}
