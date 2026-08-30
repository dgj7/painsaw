use crate::config::input_config::mc::MouseHandler;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::input::keyboard::kin::KeyInputName;
use crate::input::keyboard::ks::KeyState;
use crate::support::timing::EngineTiming;
use crate::WorldController;
use std::collections::HashMap;

///
/// handle keys via the configured key handler.
///
pub(crate) fn handle_key_change<T: KeyHandler + MouseHandler + WorldController + 'static>(
    name: &KeyInputName,
    state: &mut KeyState,
    game: &T,
    camera: &mut Camera,
    config: &EngineConfig,
    timing: &EngineTiming,
) {
    match name {
        KeyInputName::KeyEscape => game.handle_escape_key_change(name, state, camera, config, timing),
        KeyInputName::KeyA => game.handle_a_key_change(name, state, camera, config, timing),
        KeyInputName::KeyD => game.handle_d_key_change(name, state, camera, config, timing),
        KeyInputName::KeyG => game.handle_g_key_change(name, state, camera, config, timing),
        KeyInputName::KeyM => game.handle_m_key_change(name, state, camera, config, timing),
        KeyInputName::KeyS => game.handle_s_key_change(name, state, camera, config, timing),
        KeyInputName::KeyW => game.handle_w_key_change(name, state, camera, config, timing),
    }
}

// todo: re-engineer key and mouse handlers so there is only one thing to call to handle them

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
    fn check_key_states(&self, _states: &HashMap<KeyInputName, KeyState>, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, ) {}
    fn handle_escape_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, ) {}
    fn handle_a_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, ) {}
    fn handle_d_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, ) {}
    fn handle_g_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, ) {}
    fn handle_m_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, ) {}
    fn handle_s_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, ) {}
    fn handle_w_key_change(&self, _name: &KeyInputName, _state: &mut KeyState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, ) {}
}
