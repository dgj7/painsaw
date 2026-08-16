use crate::d1::Demo1;
use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::geometry::orient::movement::spectator::SpectatorMovementStrategy;
use engine::graphics::camera::Camera;
use engine::input::mouse::md::MouseDelta;
use engine::input::screen::ScreenState;
use engine::support::timing::EngineTiming;
use engine::window::api::mc::move_cursor;

impl MouseHandler for Demo1 {

    fn handle_mouse_deltas(
        &self,
        deltas: &Vec<MouseDelta>,
        camera: &mut Camera,
        config: &EngineConfig,
        _timing: &EngineTiming,
        screen: &mut ScreenState,
    ) {
        /* update mouse look  */
        <Demo1 as SpectatorMovementStrategy>::update_look(deltas, camera, config);

        /* compute center and move cursor */
        let center = &screen.window_center;
        move_cursor(center);
    }
}


