use engine::support::image::bitmap::{load_bitmap_from_bytes};
use engine::window::context::RendererContext;
use engine::window::wc::WorldController;

const B24_BMP: &[u8] = include_bytes!("../assets/24b.bmp");

pub(crate) struct Demo2WorldController {}

impl WorldController for Demo2WorldController {
    fn initialize_world_helper(&self, _context: &mut RendererContext) {
        load_bitmap_from_bytes(B24_BMP).expect("failed to load ../assets/24b.bmp");
    }

    fn update_world_helper(&self, _context: &mut RendererContext) {
        // todo
    }
}

impl Demo2WorldController {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
