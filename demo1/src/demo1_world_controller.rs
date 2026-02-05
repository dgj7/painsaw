use crate::d1m2d::{create_2d_axes, create_2d_grid_x_lines, create_2d_grid_y_lines, create_2d_repeated_texts};
use crate::d1m3d::{create_3d_axes, create_3d_cuboid_1, create_3d_cuboid_wall_2};
use engine::input::r#in::InputName;
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::window::context::RendererContext;
use engine::window::wc::WorldController;

static M2D_XY_PURPLE: &str = "1-2d-xy-purple";
static M2D_X_HORIZ: &str = "2-2d-x-horizontal";
static M2D_Y_VERT: &str = "2-2d-y-vertical";

pub(crate) struct Demo1WorldController {}

impl WorldController<f32> for Demo1WorldController {
    fn initialize_world_helper(&self, context: &mut RendererContext<f32>) {
        /* 2d */
        context.g2d.attach(M2D_XY_PURPLE, create_2d_axes(&context.camera));
        context.g2d.attach(M2D_X_HORIZ, create_2d_grid_x_lines(&context.camera));
        context.g2d.attach(M2D_Y_VERT, create_2d_grid_y_lines(&context.camera));
        context.g2d.attach("99-repeated", create_2d_repeated_texts(16, 0.0, 710.0));

        /* 3d */
        context.g3d.attach("4-3d-axes", create_3d_axes());
        context.g3d.attach("6-3d-cuboid-1", create_3d_cuboid_1());
        context.g3d.attach("6-3d-cuboid-wall-2", create_3d_cuboid_wall_2());
    }

    fn update_world_helper(&self, context: &mut RendererContext<f32>) {
        match context.input.clone().lock() {
            Ok(uin) => {
                /* gather some variables */
                let ccd = uin.current_client_dimensions.clone();

                /* handle window resize for grid */
                if uin.screen_resized {
                    context.g2d.update(M2D_XY_PURPLE, |e| *e = create_2d_axes(&context.camera));
                    context.g2d.update(M2D_X_HORIZ, |e| *e = create_2d_grid_x_lines(&context.camera));
                    context.g2d.update(M2D_Y_VERT, |e| *e = create_2d_grid_y_lines(&context.camera));

                    log(LogLevel::Debug, &|| String::from(format!("window size changed ({}x{}); 2d storage count is [{}]", ccd.width, ccd.height, context.g2d.count())));
                }

                /* camera controls */
                if let Some(wk) = uin.states.get(&InputName::KeyW) && wk.current.is_active() {
                    context.camera.orientation.move_forward(&context.config, &context.timing);
                }
                if let Some(sk) = uin.states.get(&InputName::KeyS) && sk.current.is_active() {
                    context.camera.orientation.move_backward(&context.config, &context.timing);
                }
                if let Some(ak) = uin.states.get(&InputName::KeyA) && ak.current.is_active() {
                    context.camera.orientation.move_left(&context.config, &context.timing);
                }
                if let Some(dk) = uin.states.get(&InputName::KeyD) && dk.current.is_active() {
                    context.camera.orientation.move_right(&context.config, &context.timing);
                }
            }
            Err(_) => {
                panic!("todo: handle mutex lock failure")
            }
        }

        context.first_frame_rendered = true;
        context.frame_count += 1;
    }
}

impl Demo1WorldController {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
