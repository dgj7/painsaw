use crate::config::EngineConfig;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::camera::Camera;
use crate::graphics::storage::Models;
use crate::input::mouse::md::MouseDelta;
use crate::input::mouse::min::MouseInputName;
use crate::input::mouse::ms::MouseState;
use crate::support::timing::EngineTiming;

///
/// handle mouse inputs.
///
pub fn handle_mouse_change<T: MouseHandler>(
    name: &MouseInputName,
    state: &mut MouseState,
    game: &mut T,
    config: &EngineConfig,
    camera: &mut Camera,
    timing: &EngineTiming,
    models: &mut Models,
) {
    match name {
        MouseInputName::MouseLeftButton => {
            models.ui.click(&Vertex2D { x: state.current.x as f32, y: state.current.y as f32 });
            game.handle_left_click(state, config, camera, timing, models);
        }
        MouseInputName::MouseRightButton => { game.handle_right_click(state, config, camera, timing, models) }
        MouseInputName::MouseScroll => {}
        MouseInputName::MouseMove => game.handle_mouse_move(state, config, camera, timing, models),
    }
}

///
/// handle mouse changes.
///
pub trait MouseHandler {
    fn handle_mouse_move(&mut self, _state: &mut MouseState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_left_click(&mut self, _state: &MouseState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_right_click(&mut self, _state: &MouseState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
    fn handle_mouse_deltas(&mut self, _deltas: &Vec<MouseDelta>, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models) {}
}
