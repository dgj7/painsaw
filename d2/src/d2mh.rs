use crate::d2::Demo2;
use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::graphics::storage::gxd::Models;
use engine::input::mouse::ms::MouseState;
use engine::support::timing::EngineTiming;

impl MouseHandler for Demo2 {
    fn handle_mouse_move(&self, _state: &mut MouseState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, _models: &mut Models,) {}
}
