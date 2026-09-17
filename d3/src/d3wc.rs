use crate::d3::Demo3;
use crate::d3ui::ui1;
use engine::graphics::camera::Camera;
use engine::graphics::storage::gxd::Models;
use engine::input::UserInput;
use engine::support::timing::EngineTiming;
use engine::WorldController;
use std::sync::{Arc, Mutex};
use engine::support::logger::log;
use engine::support::logger::log_level::LogLevel;

impl WorldController for Demo3 {
    fn initialize_world_helper(&self, camera: &Camera, models: &mut Models) {
        /* 2d */
        models.ui.add(1, ui1(camera));

        /* enable the first ui */
        models.ui.activate(1);
    }

    fn update_world_helper(&self, input: Arc<Mutex<UserInput>>, camera: &Camera, _timing: &mut EngineTiming, models: &mut Models) {
        match input.clone().lock() {
            Ok(uin) => {
                /* gather some variables */
                let ccd = camera.screen.current_client_dimensions.clone();

                /* handle window resize for grid */
                if uin.screen_resized {
                    models.ui.resize(&camera.screen, &mut models.g2d);

                    log(LogLevel::Debug, &|| { String::from(format!("window size changed ({}x{}); 2d storage count is [{}]", ccd.width, ccd.height, models.g2d.count())) });
                }
            }
            Err(_) => {
                panic!("todo: handle mutex lock failure")
            }
        }
    }
}
