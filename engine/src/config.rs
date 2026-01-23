//! 
//! storage for engine configurations.
//!

use num_traits::Float;
use crate::config::input_config::InputConfig;
use crate::config::renderer_config::RendererConfig;
use crate::config::window_config::WindowConfig;

pub mod window_config;
pub mod renderer_config;
pub mod input_config;

pub struct EngineConfig<F: Float> {
    pub window: WindowConfig,
    pub renderer: RendererConfig,
    pub input: InputConfig<F>,
}
