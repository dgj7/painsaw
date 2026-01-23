use engine::config::EngineConfig;
use engine::config::renderer_config::RendererConfig;
use engine::config::window_config::{WindowConfig, WindowDimensions};
use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};

pub fn create_engine_config() -> EngineConfig {
    EngineConfig {
        window: WindowConfig {
            dimensions: WindowDimensions::Dimensional {
                width: 1920,
                height: 1080,
            },
            title: Some(String::from("Demo1 - MsWin/OpenGL")),
            wndclass: Some(String::from("PAINSAW-DEMO1")),
        },
        renderer: RendererConfig {
            graphics: GraphicsSubSystem::OpenGL {
                pipeline: OpenGLPipeline::FixedFunction,
            },
        },
    }
}
