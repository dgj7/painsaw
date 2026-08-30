use crate::config::input_config::kc::KeyHandler;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::input::mouse::md::MouseDelta;
use crate::input::mouse::min::MouseInputName;
use crate::input::mouse::ms::MouseState;
use crate::input::screen::ScreenState;
use crate::support::timing::EngineTiming;
use crate::WorldController;

///
/// handle mouse inputs.
///
pub fn handle_mouse_change<T: KeyHandler + MouseHandler + WorldController + 'static>(
    game: &T,
    name: &MouseInputName,
    state: &mut MouseState,
    camera: &mut Camera,
    config: &EngineConfig,
    timing: &EngineTiming,
    screen: &mut ScreenState,
) {
    match name {
        MouseInputName::MouseLeftButton => {
            game.handle_left_click(state, camera, config, timing, screen)
        }
        MouseInputName::MouseRightButton => {
            game.handle_right_click(state, camera, config, timing, screen)
        }
        MouseInputName::MouseScroll => {}
        MouseInputName::MouseMove => game.handle_mouse_move(state, camera, config, timing, screen),
    }
}

///
/// handle mouse changes.
///
pub trait MouseHandler {
    fn handle_mouse_move(
        &self,
        _state: &mut MouseState,
        _camera: &mut Camera,
        _config: &EngineConfig,
        _timing: &EngineTiming,
        _screen: &mut ScreenState,
    ) {
    }
    fn handle_left_click(
        &self,
        _state: &MouseState,
        _camera: &mut Camera,
        _config: &EngineConfig,
        _timing: &EngineTiming,
        _screen: &mut ScreenState,
    ) {
    }
    fn handle_right_click(
        &self,
        _state: &MouseState,
        _camera: &mut Camera,
        _config: &EngineConfig,
        _timing: &EngineTiming,
        _screen: &mut ScreenState,
    ) {
    }

    fn handle_mouse_deltas(
        &self,
        _deltas: &Vec<MouseDelta>,
        _camera: &mut Camera,
        _config: &EngineConfig,
        _timing: &EngineTiming,
        _screen: &mut ScreenState,
    ) {
    }
}
