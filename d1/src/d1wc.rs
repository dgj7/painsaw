use crate::d1::Demo1;
use crate::d1m2d::{create_2d_axes, create_2d_crosshairs, create_2d_greyed_overlay, create_2d_grid_x_lines, create_2d_grid_y_lines};
use crate::d1m3d::{
    create_3d_axes, create_3d_cuboid_1, create_3d_cuboid_wall_2, create_3d_enclosing_box,
};
use engine::graphics::camera::Camera;
use engine::graphics::storage::gxd::Models;
use engine::input::UserInput;
use engine::support::logger::log;
use engine::support::logger::log_level::LogLevel;
use engine::support::timing::EngineTiming;
use engine::WorldController;
use std::sync::{Arc, Mutex};

static M2D_XY_PURPLE: &str = "1-2d-xy-purple";
static M2D_X_HORIZ: &str = "2-2d-x-horizontal";
static M2D_Y_VERT: &str = "2-2d-y-vertical";
static M2D_CROSSHAIRS: &str = "999-2d-crosshairs";
pub static M2D_OVERLAY: &str = "zzz-overlay";

impl WorldController for Demo1 {
    fn initialize_world_helper(&self, camera: &Camera, models: &mut Models) {
        /* 2d */
        models.g2d.attach(M2D_XY_PURPLE, create_2d_axes(&camera));
        models.g2d.attach(M2D_CROSSHAIRS, create_2d_crosshairs(&camera));
        models.g2d.attach(M2D_OVERLAY, create_2d_greyed_overlay(&camera));

        //context.g2d.attach(M2D_X_HORIZ, create_2d_grid_x_lines(&context.camera));
        //context.g2d.attach(M2D_Y_VERT, create_2d_grid_y_lines(&context.camera));
        //context.g2d.attach("99-repeated", create_2d_repeated_texts(16, 0.0, 710.0));

        /* 3d */
        models.g3d.attach("4-3d-axes", create_3d_axes());
        models.g3d.attach("6-3d-cuboid-1", create_3d_cuboid_1());
        models.g3d.attach("6-3d-cuboid-wall-2", create_3d_cuboid_wall_2());
        models.g3d.attach("6-3d-cuboid-enclosing", create_3d_enclosing_box());
    }

    fn update_world_helper(
        &self,
        input: Arc<Mutex<UserInput>>,
        camera: &Camera,
        timing: &mut EngineTiming,
        models: &mut Models,
    ) {
        match input.clone().lock() {
            Ok(uin) => {
                /* gather some variables */
                let ccd = camera.screen.current_client_dimensions.clone();

                /* handle window resize for grid */
                if uin.screen_resized {
                    models.g2d.update(M2D_XY_PURPLE, |e| *e = create_2d_axes(&camera));
                    models.g2d.update(M2D_X_HORIZ, |e| *e = create_2d_grid_x_lines(&camera));
                    models.g2d.update(M2D_Y_VERT, |e| *e = create_2d_grid_y_lines(&camera));
                    models.g2d.update(M2D_CROSSHAIRS, |e| *e = create_2d_crosshairs(&camera));
                    models.g2d.update(M2D_OVERLAY, |e| *e = create_2d_greyed_overlay(&camera));

                    log(LogLevel::Debug, &|| { String::from(format!("window size changed ({}x{}); 2d storage count is [{}]", ccd.width, ccd.height, models.g2d.count())) });
                }
            }
            Err(_) => {
                panic!("todo: handle mutex lock failure")
            }
        }

        timing.first_frame_rendered = true;
        timing.frame_count += 1;
    }
}
