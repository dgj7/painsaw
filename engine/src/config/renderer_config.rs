use crate::graphics::subsystem::GraphicsSubSystem;

pub struct RendererConfig {
    pub graphics: GraphicsSubSystem,
}

impl RendererConfig {
    pub fn new(grss: GraphicsSubSystem) -> RendererConfig {
        RendererConfig {
            graphics: grss,
        }
    }
}
