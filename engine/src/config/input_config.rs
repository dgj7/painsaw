use std::collections::HashMap;
use num_traits::Float;
use crate::input::r#in::InputName;
use crate::input::is::InputState;
use crate::window::context::RendererContext;

pub struct InputConfig<F: Float> {
    pub behaviors: HashMap<InputName, fn(&mut RendererContext<F>, &InputState)>,
}

impl<F: Float> InputConfig<F> {
    pub fn new(behaviors: HashMap<InputName, fn(&mut RendererContext<F>, &InputState)>) -> InputConfig<F> {
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
