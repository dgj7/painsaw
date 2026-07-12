use engine::config::input_config::InputConfig;
use engine::config::move_config::MoveConfig;
use engine::config::renderer_config::RendererConfig;
use engine::config::window_config::{WindowConfig, WindowDimensions};
use engine::config::EngineConfig;
use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};
use std::sync::Arc;
use engine::config::input_config::mh::DefaultMouseHandler;
use crate::d2ki::KeyInputs;

pub fn create_engine_config() -> EngineConfig {
    EngineConfig::new(
        WindowConfig {
            dimensions: WindowDimensions::Dimensional {
                width: 1920,
                height: 1080,
            },
            title: Some(String::from("Demo2 - MsWin/OpenGL")),
            window_id: Some(String::from("PAINSAW-DEMO2")),
        },
        RendererConfig {
            graphics: GraphicsSubSystem::OpenGL {
                pipeline: OpenGLPipeline::FixedFunction,
            },
            show_fps: true,
            show_cam_coords: false,
            fps_cap: Some(240),
        },
        InputConfig {
            key_handler: Arc::new(KeyInputs {}),
            mouse_handler: Arc::new(DefaultMouseHandler{}),
        },
        MoveConfig {
            forward_speed: 2.0,
            backward_speed: 2.0,
            ..Default::default()
        },
    )
}
