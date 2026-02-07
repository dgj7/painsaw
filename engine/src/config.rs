//! 
//! storage for engine configurations.
//!

use std::collections::BTreeMap;
use crate::config::command_config::CommandConfig;
use crate::config::input_config::InputConfig;
use crate::config::move_config::MoveConfig;
use crate::config::renderer_config::RendererConfig;
use crate::config::window_config::WindowConfig;

pub mod window_config;
pub mod renderer_config;
pub mod input_config;
pub mod move_config;
pub mod command_config;

/* cvar keys */
pub static CVAR_FOV: &str = "cvar-fov";

/* cvar defaults */
pub static DEFAULT_FOV: f64 = 45.0;

pub struct EngineConfig {
    pub window: WindowConfig,
    pub renderer: RendererConfig,
    pub input: InputConfig,
    pub movement: MoveConfig,
    pub command: CommandConfig,

    cvars: BTreeMap<String, String>,
}

impl EngineConfig {
    pub fn get_cvar<FN, O>(&self, name: &str, f: FN) -> Option<O>
    where
        FN: Fn(&str) -> O,
    {
        let maybe = self.cvars.get(name);
        maybe.map(|x| f(x))
    }
}

impl EngineConfig {
    pub fn new(window: WindowConfig, renderer: RendererConfig, input: InputConfig, movement: MoveConfig, command: CommandConfig) -> Self {
        EngineConfig {
            window,
            renderer,
            input,
            movement,
            command,
            cvars: create_initial_cvars(),
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
            command: CommandConfig::default(),
            cvars: create_initial_cvars(),
        }
    }
}

fn create_initial_cvars() -> BTreeMap<String, String> {
    BTreeMap::from([
        (CVAR_FOV.to_string(), DEFAULT_FOV.to_string()),
    ])
}
