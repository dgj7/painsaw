use crate::input::Inputs;
use engine::config::input_config::InputConfig;
use engine::config::move_config::MoveConfig;
use engine::config::renderer_config::RendererConfig;
use engine::config::window_config::{WindowConfig, WindowDimensions};
use engine::config::EngineConfig;
use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};
use std::sync::Arc;

pub fn create_engine_config() -> EngineConfig {
    EngineConfig::new(
        WindowConfig {
            dimensions: WindowDimensions::Dimensional {
                width: 1920,
                height: 1080,
            },
            title: Some(String::from("Demo1 - MsWin/OpenGL")),
            window_id: Some(String::from("PAINSAW-DEMO1")),
        },
        RendererConfig {
            graphics: GraphicsSubSystem::OpenGL {
                pipeline: OpenGLPipeline::FixedFunction,
            },
            show_fps: true,
            show_cam_coords: true,
            fps_cap: Some(60),
        },
        InputConfig {
            behaviors: Arc::new(Inputs{}),
        },
        MoveConfig {
            forward_speed: 2.0,
            backward_speed: 2.0,
            ..Default::default()
        },
    )
}
