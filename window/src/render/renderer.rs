use crate::render::render_context::RendererContext;

pub trait Renderer {
    // todo: add update method, to update world state
    fn render_scene(&self, context: &mut RendererContext);
}
