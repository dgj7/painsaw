use crate::config::input_config::kc::KeyHandler;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::storage::gxd::Models;
use crate::input::mouse::md::MouseDelta;
use crate::input::mouse::min::MouseInputName;
use crate::input::mouse::ms::MouseState;
use crate::support::timing::EngineTiming;
use crate::WorldController;

///
/// handle mouse inputs.
///
pub fn handle_mouse_change<T: KeyHandler + MouseHandler + WorldController + 'static>(
    name: &MouseInputName,
    state: &mut MouseState,
    game: &T,
    config: &EngineConfig,
    camera: &mut Camera,
    timing: &EngineTiming,
    models: &Models,
) {
    match name {
        MouseInputName::MouseLeftButton => { game.handle_left_click(state, config, camera, timing, models) }
        MouseInputName::MouseRightButton => { game.handle_right_click(state, config, camera, timing, models) }
        MouseInputName::MouseScroll => {}
        MouseInputName::MouseMove => game.handle_mouse_move(state, config, camera, timing, models),
    }
}

///
/// handle mouse changes.
///
pub trait MouseHandler {
    fn handle_mouse_move(&self, _state: &mut MouseState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &Models) {}
    fn handle_left_click(&self, _state: &MouseState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &Models) {}
    fn handle_right_click(&self, _state: &MouseState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &Models) {}
    fn handle_mouse_deltas(&self, _deltas: &Vec<MouseDelta>, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &Models) {}
}
