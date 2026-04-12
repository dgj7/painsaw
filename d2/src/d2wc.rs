use crate::d2m2d::{create_2d_bmp_24b, create_2d_tga};
use engine::window::context::RendererContext;
use engine::window::wc::WorldController;

static M2D_BMP_24B: &str = "bmp-24b";
static M2D_TGA: &str = "tga";

pub(crate) struct Demo2WorldController {}

impl WorldController for Demo2WorldController {
    fn initialize_world_helper(&self, context: &mut RendererContext) {
        context.g2d.attach(M2D_BMP_24B, create_2d_bmp_24b());
        context.g2d.attach(M2D_TGA, create_2d_tga());
    }

    fn update_world_helper(&self, context: &mut RendererContext) {
        context.first_frame_rendered = true;
        context.frame_count += 1;
    }
}

impl Demo2WorldController {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
