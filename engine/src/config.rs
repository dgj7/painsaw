//! 
//! storage for engine configurations.
//!

use crate::config::renderer_config::RendererConfig;
use crate::config::window_config::WindowConfig;

pub mod window_config;
pub mod renderer_config;

pub struct EngineConfig {
    pub window: WindowConfig,
    pub renderer: RendererConfig,
}
