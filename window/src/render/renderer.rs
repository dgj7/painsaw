use crate::render::render_context::RendererContext;

pub trait Renderer {
    fn render_scene(&self, context: &mut RendererContext);
}
