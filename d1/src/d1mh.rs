use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::mouse::ms::MouseState;
use engine::input::screen::ScreenState;
use engine::support::timing::EngineTiming;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, _state: &mut MouseState, _camera: &mut Camera, _config: &EngineConfig, _timing: &EngineTiming, _screen: &mut ScreenState) {
        /*
        if state.current.handled {
            return;
        }

        if state.previous.is_some() {
            state.enabled = false;


            camera.orientation.yaw = camera.orientation.yaw + (dx * config.input.mouse_sensitivity * -1.0);
            camera.orientation.pitch = camera.orientation.pitch + (dy * config.input.mouse_sensitivity * -1.0);

            if camera.orientation.pitch > 89.0 { camera.orientation.pitch = 89.0; }
            if camera.orientation.pitch < -89.0 { camera.orientation.pitch = -89.0; }


            let center = &screen.window_center;
            move_cursor(center);
            state.previous = None;
            state.enabled = true;
        }
        */
    }
}
