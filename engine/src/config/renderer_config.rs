use crate::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};

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

impl Default for RendererConfig {
    fn default() -> RendererConfig {
        RendererConfig {
            graphics: GraphicsSubSystem::OpenGL { pipeline: OpenGLPipeline::FixedFunction },
        }
    }
}
