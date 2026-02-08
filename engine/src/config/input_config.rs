pub mod kc;
pub mod mc;

pub use crate::config::input_config::kc::{DefaultKeyHandler, KeyHandler};
use std::sync::Arc;
use crate::config::input_config::mc::{DefaultMouseHandler, MouseHandler};

pub struct InputConfig {
    pub key_handler: Arc<dyn KeyHandler>,
    pub mouse_handler: Arc<dyn MouseHandler>,
}

impl InputConfig {
    pub fn new(key_handler: Arc<dyn KeyHandler>, mouse_handler: Arc<dyn MouseHandler>) -> InputConfig {
        InputConfig {
            key_handler,
            mouse_handler,
        }
    }
}

///
/// default input config.
///
impl Default for InputConfig {
    fn default() -> InputConfig {
        InputConfig {
            key_handler: Arc::new(DefaultKeyHandler { }),
            mouse_handler: Arc::new(DefaultMouseHandler { }),
        }
    }
}
