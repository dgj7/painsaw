use crate::d1cmd::Command::{CameraMoveBackward, CameraMoveForward, CameraStrafeLeft, CameraStrafeRight};
use crate::d1cmd::{handle_command, Command};
use engine::config::input_config::KeyHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::keyboard::kin::KeyInputName;
use engine::input::keyboard::kin::KeyInputName::{KeyA, KeyD, KeyS, KeyW};
use engine::input::keyboard::ks::KeyState;
use engine::support::logger::log;
use engine::support::logger::log_level::LogLevel;
use engine::support::timing::EngineTiming;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use crate::d1::Demo1;

static KEYS: LazyLock<Mutex<HashMap<KeyInputName, Command>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(KeyW, CameraMoveForward);
    map.insert(KeyS, CameraMoveBackward);
    map.insert(KeyA, CameraStrafeLeft);
    map.insert(KeyD, CameraStrafeRight);
    Mutex::new(map)
});

impl KeyHandler for Demo1 {
    fn check_key_states(&self, states: &HashMap<KeyInputName, KeyState>, camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
        states.into_iter()
            .filter(|(_, input_state)| input_state.current.is_active())
            .for_each(|(key_name, _)| {
                if let Ok(mutex) = KEYS.lock() {
                    if let Some(command) = mutex.get(&key_name) {
                        handle_command(&command, &mut camera.orientation, &config, &timing);
                    }
                }
            });
    }

    fn handle_g_key_change(&self, name: &KeyInputName, state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {
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
