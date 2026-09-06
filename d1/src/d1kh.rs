use crate::d1::Demo1;
use crate::d1cmd::Command::{
    CameraMoveBackward, CameraMoveForward, CameraStrafeLeft, CameraStrafeRight,
};
use crate::d1cmd::{handle_command, Command};
use engine::config::input_config::kc::KeyHandler;
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
use engine::graphics::storage::gxd::Models;
use crate::d1wc::M2D_OVERLAY;

static KEYS: LazyLock<Mutex<HashMap<KeyInputName, Command>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(KeyW, CameraMoveForward);
    map.insert(KeyS, CameraMoveBackward);
    map.insert(KeyA, CameraStrafeLeft);
    map.insert(KeyD, CameraStrafeRight);
    Mutex::new(map)
});

impl KeyHandler for Demo1 {
    fn check_key_states(
        &mut self,
        _states: &HashMap<KeyInputName, KeyState>,
        _config: &EngineConfig,
        _camera: &mut Camera,
        _timing: &EngineTiming,
        _models: &mut Models,
    ) {
        _states
            .into_iter()
            .filter(|(_, input_state)| input_state.current.is_active())
            .for_each(|(key_name, _)| {
                if let Ok(mutex) = KEYS.lock() {
                    if let Some(command) = mutex.get(&key_name) {
                        handle_command::<Demo1>(&command, _camera, &_config, &_timing);
                    }
                }
            });
    }

    fn handle_escape_key_change(&mut self, _name: &KeyInputName, state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.g2d.update(M2D_OVERLAY, |m| {
            if !state.current.is_handled() && state.current.is_active() {
                state.current.set_handled();
                m.visible = !m.visible;
            }
        });
    }

    fn handle_g_key_change(
        &mut self,
        _name: &KeyInputName,
        _state: &mut KeyState,
        _config: &EngineConfig,
        _camera: &mut Camera,
        _timing: &EngineTiming,
        _models: &mut Models,
    ) {
        let duration = _state.previous_key_state_duration();
        log(LogLevel::Debug, &|| {
            String::from(format!(
                "{}: {}    ({} for {}ms)",
                _name,
                _state.current,
                _state.previous,
                duration.as_millis()
            ))
        });
    }
}
