use window::render::render_context::RendererContext;
use window::render::renderer::Renderer;

pub(crate) struct GameRenderer {}

impl Renderer for GameRenderer {
    fn render_scene(&self, context: &mut RendererContext) {
        context.first_frame_rendered = true;
        context.frame_count+=1;
    }
}

impl GameRenderer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
