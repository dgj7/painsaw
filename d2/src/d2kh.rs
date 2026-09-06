use crate::d2::Demo2;
use engine::config::input_config::kc::KeyHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::keyboard::kin::KeyInputName;
use engine::input::keyboard::ks::KeyState;
use engine::support::timing::EngineTiming;
use std::collections::HashMap;
use engine::graphics::storage::gxd::Models;

impl KeyHandler for Demo2 {
    fn check_key_states(&mut self, _states: &HashMap<KeyInputName, KeyState>, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models,) {}
    fn handle_g_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models,) {}
}
