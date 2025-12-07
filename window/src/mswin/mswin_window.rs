use logger::log;
use logger::model::log_config::LoggerConfig;
use logger::model::log_level::LogLevel;
use crate::model::window::Window;
use crate::model::window_config::WindowConfig;

pub struct MsWinWindow {
    // todo
}

impl Window for MsWinWindow {
    fn begin_display(&self, logger: &LoggerConfig) {
        log(&logger, LogLevel::Debug, &|| "begin window display");
    }
}

impl MsWinWindow {
    pub fn new(_request : &WindowConfig) -> Box<dyn Window> {
        Box::new(MsWinWindow {})
    }
}
