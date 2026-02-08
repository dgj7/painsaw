use engine::config::input_config::KeyHandler;
use engine::input::r#in::InputName;
use engine::input::is::InputState;
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::window::context::RendererContext;
use std::collections::HashMap;

pub(crate) struct Inputs {}

impl KeyHandler for Inputs {
    fn check_key_states(&self, states: &HashMap<InputName, InputState>, context: &mut RendererContext) {
        /* camera controls */
        if let Some(wk) = states.get(&InputName::KeyW) && wk.current.is_active() {
            context.camera.orientation.move_forward(&context.config, &context.timing);
        }
        if let Some(sk) = states.get(&InputName::KeyS) && sk.current.is_active() {
            context.camera.orientation.move_backward(&context.config, &context.timing);
        }
        if let Some(ak) = states.get(&InputName::KeyA) && ak.current.is_active() {
            context.camera.orientation.move_left(&context.config, &context.timing);
        }
        if let Some(dk) = states.get(&InputName::KeyD) && dk.current.is_active() {
            context.camera.orientation.move_right(&context.config, &context.timing);
        }
    }

    fn handle_g_key_change(&self, name: &InputName, state: &mut InputState, _context: &mut RendererContext) {
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
