use engine::config::input_config::KeyHandler;
use engine::input::keyboard::ks::KeyState;
use engine::input::keyboard::kin::KeyInputName;
use engine::PainsawContext;
use std::collections::HashMap;

pub(crate) struct KeyInputs {}

impl KeyHandler for KeyInputs {
    fn check_key_states(&self, _states: &HashMap<KeyInputName, KeyState>, _context: &mut PainsawContext) {
        // todo
    }

    fn handle_g_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _context: &mut PainsawContext) {
        // todo
    }
}
