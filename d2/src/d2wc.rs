use crate::d2::Demo2;
use crate::d2m2d::{create_2d_bmp_24b, create_2d_tga_32b_b2t_l2r};
use engine::graphics::camera::Camera;
use engine::graphics::storage::g2d::Graph2D;
use engine::graphics::storage::g3d::Graph3D;
use engine::input::screen::ScreenState;
use engine::input::UserInput;
use engine::support::timing::EngineTiming;
use engine::WorldController;
use std::sync::{Arc, Mutex};

static M2D_BMP_24B: &str = "bmp-24b";
static M2D_TGA_32B_B2T_L2R: &str = "tga-32b-b2t-l2r";

impl WorldController for Demo2 {
    fn initialize_world_helper(&self, _camera: &Camera, g2d: &mut Graph2D, _g3d: &mut Graph3D) {
        g2d.attach(M2D_BMP_24B, create_2d_bmp_24b());
        g2d.attach(M2D_TGA_32B_B2T_L2R, create_2d_tga_32b_b2t_l2r());
    }

    fn update_world_helper(
        &self,
        _input: Arc<Mutex<UserInput>>,
        _screen: &ScreenState,
        _camera: &Camera,
        timing: &mut EngineTiming,
        _g2d: &mut Graph2D,
        _g3d: &mut Graph3D,
    ) {
        timing.first_frame_rendered = true;
        timing.frame_count += 1;
    }
}
