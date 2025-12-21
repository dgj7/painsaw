use crate::render::model::render_context::RendererContext;

pub trait Renderer {
    fn update_world(&self, context: &mut RendererContext);
    fn before_render(&self, context: &mut RendererContext);
    fn render_2d_scene(&self, context: &mut RendererContext);
    fn render_3d_scene(&self, context: &mut RendererContext);

    fn after_render(&self, context: &mut RendererContext);
}
