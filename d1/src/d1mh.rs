use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::mouse::ms::MouseState;
use engine::support::timing::EngineTiming;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, state: &mut MouseState, camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
        camera.orientation.look(state, &config, &timing);
        //center_mouse()
    }
}
