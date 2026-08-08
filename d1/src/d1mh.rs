use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::mouse::md::MouseDelta;
use engine::input::screen::ScreenState;
use engine::support::timing::EngineTiming;
use engine::window::api::mc::move_cursor;
use crate::d1::Demo1;

impl MouseHandler for Demo1 {

    fn handle_mouse_deltas(
        &self,
        deltas: &Vec<MouseDelta>,
        camera: &mut Camera,
        config: &EngineConfig,
        _timing: &EngineTiming,
        screen: &mut ScreenState,
    ) {
        let dx = deltas.iter().map(|d| d.dx).sum::<f32>();
        let dy = deltas.iter().map(|d| d.dy).sum::<f32>();

        camera.orientation.yaw = camera.orientation.yaw + (dx * config.input.mouse_sensitivity * -1.0);
        camera.orientation.pitch = camera.orientation.pitch + (dy * config.input.mouse_sensitivity * -1.0);

        if camera.orientation.pitch > 89.0 { camera.orientation.pitch = 89.0; }
        if camera.orientation.pitch < -89.0 { camera.orientation.pitch = -89.0; }

        let center = &screen.window_center;
        move_cursor(center);
    }
}
