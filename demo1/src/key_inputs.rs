use engine::config::input_config::KeyHandler;
use engine::input::kin::KeyInputName;
use engine::input::is::InputState;
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::window::context::RendererContext;
use std::collections::HashMap;

pub(crate) struct KeyInputs {}

impl KeyHandler for KeyInputs {
    fn check_key_states(&self, states: &HashMap<KeyInputName, InputState>, context: &mut RendererContext) {
        /* camera controls */
        if let Some(wk) = states.get(&KeyInputName::KeyW) && wk.current.is_active() {
            context.camera.orientation.move_forward(&context.config, &context.timing);
        }
        if let Some(sk) = states.get(&KeyInputName::KeyS) && sk.current.is_active() {
            context.camera.orientation.move_backward(&context.config, &context.timing);
        }
        if let Some(ak) = states.get(&KeyInputName::KeyA) && ak.current.is_active() {
            context.camera.orientation.move_left(&context.config, &context.timing);
        }
        if let Some(dk) = states.get(&KeyInputName::KeyD) && dk.current.is_active() {
            context.camera.orientation.move_right(&context.config, &context.timing);
        }
    }

    fn handle_g_key_change(&self, name: &KeyInputName, state: &mut InputState, _context: &mut RendererContext) {
        let duration = state.previous_key_state_duration();
        log(LogLevel::Debug, &|| {
            String::from(format!(
                "{}: {}    ({} for {}ms)",
                name,
                state.current,
                state.previous,
                duration.as_millis()
            ))
        });
    }
}
