use crate::model::window::Window;
use crate::model::window_config::WindowConfig;
use logger::model::log_config::LoggerConfig;

pub struct MsWinWindow {
    // todo
}

impl Window for MsWinWindow {
    fn begin_display(&self, logger: &LoggerConfig) {
        logger.debug(&|| "begin window display");
    }
}

impl MsWinWindow {
    pub fn new(_request : &WindowConfig) -> Box<dyn Window> {
        Box::new(MsWinWindow {})
    }
}
