use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::mouse::ms::MouseState;
use engine::input::screen::ScreenState;
use engine::support::timing::EngineTiming;
use crate::d2::Demo2;

impl MouseHandler for Demo2 {
    fn handle_mouse_move(&self, _state: &mut MouseState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, _screen: &mut ScreenState) {}
}
