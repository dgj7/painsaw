use window::render::render_context::RendererContext;
use window::render::renderer::Renderer;

pub(crate) struct GameRenderer {}

impl Renderer for GameRenderer {
    fn render_scene(&self, _context: &RendererContext) {
        // nothing yet
    }
}

impl GameRenderer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
