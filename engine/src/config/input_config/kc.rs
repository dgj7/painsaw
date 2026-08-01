use crate::input::keyboard::kin::KeyInputName;
use crate::input::keyboard::ks::KeyState;
use std::collections::HashMap;
use std::sync::Arc;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::support::timing::EngineTiming;

///
/// handle keys via the configured key handler.
///
pub(crate) fn handle_key_change(handler: Arc<dyn KeyHandler>, name: &KeyInputName, state: &mut KeyState, camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
    match name {
        KeyInputName::KeyEscape => handler.handle_escape_key_change(name, state, camera, config, timing),
        KeyInputName::KeyA => handler.handle_a_key_change(name, state, camera, config, timing),
        KeyInputName::KeyD => handler.handle_d_key_change(name, state, camera, config, timing),
        KeyInputName::KeyG => handler.handle_g_key_change(name, state, camera, config, timing),
        KeyInputName::KeyM => handler.handle_m_key_change(name, state, camera, config, timing),
        KeyInputName::KeyS => handler.handle_s_key_change(name, state, camera, config, timing),
        KeyInputName::KeyW => handler.handle_w_key_change(name, state, camera, config, timing),
    }
}

///
/// core key handler trait.
///
pub trait KeyHandler {
    ///
    /// check key states.
    ///
    /// this is useful for handling scenarios where holding a key down might not be a
    /// "new" change, but still needs to be handled as input for some games.
    ///
    fn check_key_states(&self, _states: &HashMap<KeyInputName, KeyState>, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_escape_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_a_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_d_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_g_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_m_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_s_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_w_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
}

///
/// default key handler struct; no fields necessary.
///
pub struct DefaultKeyHandler {}

///
/// identify default key handler as a key handler.
///
impl KeyHandler for DefaultKeyHandler {}
