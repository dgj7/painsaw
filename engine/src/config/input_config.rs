use std::collections::HashMap;
use crate::input::r#in::InputName;
use crate::input::is::InputState;
use crate::window::context::RendererContext;

pub struct InputConfig {
    pub behaviors: HashMap<InputName, fn(&mut RendererContext, &InputState)>,
}

impl InputConfig {
    pub fn new(behaviors: HashMap<InputName, fn(&mut RendererContext, &InputState)>) -> InputConfig {
        InputConfig {
            behaviors,
        }
    }
}

impl Default for InputConfig {
    fn default() -> InputConfig {
        InputConfig {
            behaviors: HashMap::new(),
        }
    }
}
