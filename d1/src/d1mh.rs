use crate::d1::Demo1;
use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::game::Game;
use engine::geometry::orient::movement::spectator::SpectatorMovementStrategy;
use engine::graphics::camera::Camera;
use engine::graphics::storage::Models;
use engine::input::mouse::md::MouseDelta;
use engine::support::timing::EngineTiming;
use engine::window::api::mc::move_cursor;

impl MouseHandler for Demo1 {
    fn handle_mouse_deltas(
        &mut self,
        deltas: &Vec<MouseDelta>,
        config: &EngineConfig,
        camera: &mut Camera,
        _timing: &EngineTiming,
        _models: &mut Models,
    ) {
        /* sc if we're displaying the menu */
        if self.is_menu() {
            return;
        }

        /* update mouse look  */
        self.update_look(deltas, config, camera);

        /* compute center and move cursor */
        let center = &camera.screen.window_center;
        move_cursor(center);
    }
}
