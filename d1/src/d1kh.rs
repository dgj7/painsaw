use engine::config::input_config::KeyHandler;
use engine::input::keyboard::kin::KeyInputName;
use engine::input::keyboard::ks::KeyState;
use engine::support::logger::log;
use engine::support::logger::log_level::LogLevel;
use engine::window::context::RendererContext;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use engine::config::EngineConfig;
use engine::geometry::orient::Orientation;
use engine::input::keyboard::kin::KeyInputName::{KeyA, KeyD, KeyS, KeyW};
use engine::support::timing::EngineTiming;
use crate::d1kh::Command::{CameraMoveBackward, CameraMoveForward, CameraStrafeLeft, CameraStrafeRight};

static KEYS: LazyLock<Mutex<HashMap<KeyInputName, Command>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(KeyW, CameraMoveForward);
    map.insert(KeyS, CameraMoveBackward);
    map.insert(KeyA, CameraStrafeLeft);
    map.insert(KeyD, CameraStrafeRight);
    Mutex::new(map)
});

pub(crate) struct KeyInputs {}

pub(crate) enum Command {
    CameraMoveForward,
    CameraStrafeLeft,
    CameraMoveBackward,
    CameraStrafeRight,
}

fn handle_command(command: &Command, orientation: &mut Orientation, ec: &EngineConfig, et: &EngineTiming) {
    match command {
        CameraMoveForward => orientation.move_forward(&ec, &et),
        CameraStrafeLeft => orientation.move_left(&ec, &et),
        CameraMoveBackward => orientation.move_backward(&ec, &et),
        CameraStrafeRight => orientation.move_right(&ec, &et),
    }
}

impl KeyHandler for KeyInputs {
    fn check_key_states(&self, states: &HashMap<KeyInputName, KeyState>, context: &mut RendererContext) {
        states.into_iter()
            .filter(|(_, input_state)| input_state.current.is_active())
            .for_each(|(key_name, _)| {
                if let Ok(mutex) = KEYS.lock() {
                    if let Some(command) = mutex.get(&key_name) {
                        handle_command(&command, &mut context.camera.orientation, &context.config, &context.timing);
                    }
                }
            });
    }

    fn handle_g_key_change(&self, name: &KeyInputName, state: &mut KeyState, _context: &mut RendererContext) {
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
