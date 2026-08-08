use engine::config::input_config::KeyHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::keyboard::kin::KeyInputName;
use engine::input::keyboard::ks::KeyState;
use engine::support::timing::EngineTiming;
use std::collections::HashMap;
use crate::d2::Demo2;

impl KeyHandler for Demo2 {
    fn check_key_states(&self, _states: &HashMap<KeyInputName, KeyState>, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_g_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
}
