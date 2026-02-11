use crate::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};

pub struct RendererConfig {
    pub graphics: GraphicsSubSystem,
    pub show_fps: bool,
    pub show_cam_coords: bool,
    pub fps_cap: Option<u16>,
}

impl RendererConfig {
    pub fn new(grss: GraphicsSubSystem) -> RendererConfig {
        RendererConfig {
            graphics: grss,

            show_fps: false,
            show_cam_coords: false,

            fps_cap: None,
        }
    }
}

impl Default for RendererConfig {
    fn default() -> RendererConfig {
        RendererConfig {
            graphics: GraphicsSubSystem::OpenGL {
                pipeline: OpenGLPipeline::FixedFunction,
            },

            show_fps: false,
            show_cam_coords: false,

            fps_cap: None,
        }
    }
}
