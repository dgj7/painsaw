use crate::render::model::render_context::RendererContext;

pub trait Renderer {
    fn update_world(&self, context: &mut RendererContext);
    fn render_scene(&self, context: &mut RendererContext);
}
