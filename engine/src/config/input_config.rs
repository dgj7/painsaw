use std::collections::HashMap;
use num_traits::Float;
use crate::input::kn::KeyName;
use crate::input::ks::KeyState;
use crate::window::context::RendererContext;

pub struct InputConfig<F: Float> {
    pub behaviors: HashMap<KeyName, fn(&mut RendererContext<F>, &KeyState)>,
}

impl<F: Float> InputConfig<F> {
    pub fn new(behaviors: HashMap<KeyName, fn(&mut RendererContext<F>, &KeyState)>) -> InputConfig<F> {
        InputConfig {
            behaviors,
        }
    }
}

impl<F: Float> Default for InputConfig<F> {
    fn default() -> InputConfig<F> {
        InputConfig {
            behaviors: HashMap::new(),
        }
    }
}
