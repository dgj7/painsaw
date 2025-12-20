use std::sync::Arc;
use crate::logger::Logger;
use crate::window::model::window_config::WindowConfig;

pub struct ApplicationConfiguration {
    pub logger: Arc<Logger>,
    pub window: WindowConfig,
}
