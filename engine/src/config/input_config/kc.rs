use crate::config::input_config::mc::MouseHandler;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::input::keyboard::kin::KeyInputName;
use crate::input::keyboard::ks::KeyState;
use crate::support::timing::EngineTiming;
use crate::WorldController;
use std::collections::HashMap;
use crate::graphics::storage::Models;

///
/// handle keys via the configured key handler.
///
pub(crate) fn handle_key_change<T: KeyHandler + MouseHandler + WorldController + 'static>(
    name: &KeyInputName,
    state: &mut KeyState,
    game: &mut T,
    config: &EngineConfig,
    camera: &mut Camera,
    timing: &EngineTiming,
    models: &mut Models,
) {
    match name {
        KeyInputName::KeyEscape => game.handle_escape_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyTilde => game.handle_tilde_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyA => game.handle_a_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyB => game.handle_b_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyC => game.handle_c_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyD => game.handle_d_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyE => game.handle_e_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyF => game.handle_f_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyG => game.handle_g_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyH => game.handle_h_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyI => game.handle_i_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyJ => game.handle_j_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyK => game.handle_k_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyL => game.handle_l_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyM => game.handle_m_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyN => game.handle_n_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyO => game.handle_o_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyP => game.handle_p_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyQ => game.handle_q_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyR => game.handle_r_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyS => game.handle_s_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyT => game.handle_t_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyU => game.handle_u_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyV => game.handle_v_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyW => game.handle_w_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyX => game.handle_x_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyY => game.handle_y_key_change(name, state, config, camera, timing, models),
        KeyInputName::KeyZ => game.handle_z_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key1 => game.handle_1_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key2 => game.handle_2_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key3 => game.handle_3_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key4 => game.handle_4_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key5 => game.handle_5_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key6 => game.handle_6_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key7 => game.handle_7_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key8 => game.handle_8_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key9 => game.handle_9_key_change(name, state, config, camera, timing, models),
        KeyInputName::Key0 => game.handle_0_key_change(name, state, config, camera, timing, models),
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
    fn check_key_states(&mut self, _states: &HashMap<KeyInputName, KeyState>, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}


    fn handle_escape_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_tilde_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_a_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_b_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_c_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_d_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_e_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_f_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_g_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_h_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_i_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_j_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_k_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_l_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_m_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_n_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_o_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_p_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_q_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_r_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_s_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_t_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_u_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_v_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_w_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_x_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_y_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_z_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_1_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_2_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_3_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_4_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_5_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_6_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_7_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_8_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_9_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_0_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
}
