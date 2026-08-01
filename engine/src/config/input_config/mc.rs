use crate::input::mouse::min::MouseInputName;
use crate::input::mouse::ms::MouseState;
use std::sync::Arc;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::support::timing::EngineTiming;

///
/// handle mouse inputs.
///
pub fn handle_mouse_change(handler: Arc<dyn MouseHandler>, name: &MouseInputName, state: &mut MouseState, camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
    match name {
        MouseInputName::MouseLeftButton => handler.handle_left_click(state, camera, config, timing),
        MouseInputName::MouseRightButton => handler.handle_right_click(state, camera, config, timing),
        MouseInputName::MouseScroll => {}
        MouseInputName::MouseMove => handler.handle_mouse_move(state, camera, config, timing),
    }
}


///
/// handle mouse changes.
///
pub trait MouseHandler {
    ///
    /// handle mouse move.
    ///
    fn handle_mouse_move(&self, _state: &mut MouseState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_left_click(&self, _state: &MouseState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
    fn handle_right_click(&self, _state: &MouseState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
}

///
/// default mouse handler.
///
pub struct DefaultMouseHandler {}

///
/// implement nothing for default mouse handler.
///
impl MouseHandler for DefaultMouseHandler {}
