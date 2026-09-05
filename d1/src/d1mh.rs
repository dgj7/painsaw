use crate::d1::Demo1;
use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::geometry::orient::movement::spectator::SpectatorMovementStrategy;
use engine::graphics::camera::Camera;
use engine::graphics::storage::gxd::Models;
use engine::input::mouse::md::MouseDelta;
use engine::support::timing::EngineTiming;
use engine::window::api::mc::move_cursor;

impl MouseHandler for Demo1 {
    fn handle_mouse_deltas(
        &self,
        _deltas: &Vec<MouseDelta>,
        _config: &EngineConfig,
        _camera: &mut Camera,
        _timing: &EngineTiming,
        _models: &Models,
    ) {
        /* update mouse look  */
        <Demo1 as SpectatorMovementStrategy>::update_look(_deltas, _camera, _config);

        /* compute center and move cursor */
        let center = &_camera.screen.window_center;
        move_cursor(center);
    }
}
