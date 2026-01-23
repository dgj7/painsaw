//! 
//! storage for engine configurations.
//!

use std::collections::BTreeMap;
use num_traits::Float;
use crate::config::input_config::InputConfig;
use crate::config::renderer_config::RendererConfig;
use crate::config::window_config::WindowConfig;

pub mod window_config;
pub mod renderer_config;
pub mod input_config;

/* cvar keys */
pub static CVAR_FOV: &str = "cvar-fov";

/* cvar defaults */
pub static DEFAULT_FOV: f64 = 45.0;

pub struct EngineConfig<F: Float> {
    pub window: WindowConfig,
    pub renderer: RendererConfig,
    pub input: InputConfig<F>,

    cvars: BTreeMap<String, String>,
}

impl<F: Float> EngineConfig<F> {
    pub fn get_cvar<FN, O>(&self, name: &str, f: FN) -> Option<O>
    where
        FN: Fn(&str) -> O,
    {
        let maybe = self.cvars.get(name);
        maybe.map(|x| f(x))
    }
}

impl<F: Float> EngineConfig<F> {
    pub fn new(window: WindowConfig, renderer: RendererConfig, input: InputConfig<F>) -> Self {
        EngineConfig {
            window,
            renderer,
            input,
            cvars: create_initial_cvars(),
        }
    }
}

impl<F: Float> Default for EngineConfig<F> {
    fn default() -> EngineConfig<F> {
        EngineConfig {
            window: WindowConfig::default(),
            renderer: RendererConfig::default(),
            input: InputConfig::default(),
            cvars: create_initial_cvars(),
        }
    }
}

fn create_initial_cvars() -> BTreeMap<String, String> {
    BTreeMap::from([
        (CVAR_FOV.to_string(), DEFAULT_FOV.to_string()),
    ])
}
