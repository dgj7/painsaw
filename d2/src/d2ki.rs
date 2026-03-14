use engine::config::input_config::KeyHandler;
use engine::input::is::InputState;
use engine::input::kin::KeyInputName;
use engine::window::context::RendererContext;
use std::collections::HashMap;

pub(crate) struct KeyInputs {}

impl KeyHandler for KeyInputs {
    fn check_key_states(&self, _states: &HashMap<KeyInputName, InputState>, _context: &mut RendererContext) {
        // todo
    }

    fn handle_g_key_change(&self, _name: &KeyInputName, _state: &mut InputState, _context: &mut RendererContext) {
        // todo
    }
}
