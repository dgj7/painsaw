use std::sync::Arc;
use crate::logger::Logger;
use crate::window::model::window_config::WindowConfig;

pub(crate) struct ApplicationConfiguration {
    pub(crate) logger: Arc<Logger>,
    pub(crate) window: WindowConfig,
}
