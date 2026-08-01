use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::mouse::ms::MouseState;
use engine::support::timing::EngineTiming;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, _state: &mut MouseState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming) {}
}
