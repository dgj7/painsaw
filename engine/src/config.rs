//!
//! storage for engine configurations.
//!

use crate::config::input_config::InputConfig;
use crate::config::move_config::MoveConfig;
use crate::config::renderer_config::RendererConfig;
use crate::config::window_config::WindowConfig;

pub mod input_config;
pub mod move_config;
pub mod renderer_config;
pub mod window_config;

pub struct EngineConfig {
    pub window: WindowConfig,
    pub renderer: RendererConfig,
    pub input: InputConfig,
    pub movement: MoveConfig,
}

impl EngineConfig {
    pub fn new(window: WindowConfig, renderer: RendererConfig, input: InputConfig, movement: MoveConfig) -> Self {
        EngineConfig {
            window,
            renderer,
            input,
            movement,
        }
    }
}

impl Default for EngineConfig {
    fn default() -> EngineConfig {
        EngineConfig {
            window: WindowConfig::default(),
            renderer: RendererConfig::default(),
            input: InputConfig::default(),
            movement: MoveConfig::default(),
        }
    }
}
