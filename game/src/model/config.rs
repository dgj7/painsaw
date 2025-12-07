use logger::model::log_config::LoggerConfig;
use window::model::window_config::WindowConfig;

pub(crate) struct ApplicationConfiguration {
    pub(crate) logger: LoggerConfig,
    pub(crate) window: WindowConfig,
}
