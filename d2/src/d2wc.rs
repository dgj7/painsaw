use crate::d2m2d::{create_2d_bmp_24b, create_2d_tga_32b_b2t_l2r};
use engine::PainsawContext;
use engine::WorldController;
use crate::d2::Demo2;

static M2D_BMP_24B: &str = "bmp-24b";
static M2D_TGA_32B_B2T_L2R: &str = "tga-32b-b2t-l2r";

impl WorldController for Demo2 {
    fn initialize_world_helper(&self, context: &mut PainsawContext) {
        context.g2d.attach(M2D_BMP_24B, create_2d_bmp_24b());
        context.g2d.attach(M2D_TGA_32B_B2T_L2R, create_2d_tga_32b_b2t_l2r());
    }

    fn update_world_helper(&self, context: &mut PainsawContext) {
        context.first_frame_rendered = true;
        context.frame_count += 1;
    }
}
