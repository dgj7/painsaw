pub mod kc;
pub mod mc;

pub use crate::config::input_config::kc::{DefaultKeyHandler, KeyHandler};
use std::sync::Arc;
use crate::config::input_config::mc::{DefaultMouseHandler, MouseHandler};

pub struct InputConfig {
    pub key_handler: Arc<dyn KeyHandler>,
    pub mouse_handler: Arc<dyn MouseHandler>,
    pub mouse_sensitivity: f32,
}

impl InputConfig {
    pub fn new(key_handler: Arc<dyn KeyHandler>, mouse_handler: Arc<dyn MouseHandler>, mouse_sensitivity: f32) -> InputConfig {
        InputConfig {
            key_handler,
            mouse_handler,
            mouse_sensitivity,
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
            mouse_sensitivity: 1.0,
        }
    }
}
