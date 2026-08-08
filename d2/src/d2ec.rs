use engine::config::input_config::mc::MouseHandler;
use engine::config::input_config::{InputConfig, KeyHandler};
use engine::config::move_config::MoveConfig;
use engine::config::renderer_config::RendererConfig;
use engine::config::window_config::{WindowConfig, WindowDimensions};
use engine::config::EngineConfig;
use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};
use std::sync::Arc;

pub fn create_engine_config<T>(core: Arc<T>) -> EngineConfig
where
    T: MouseHandler + KeyHandler + 'static
{
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
            show_screen_stats: false,
            fps_cap: Some(240),
        },
        InputConfig {
            key_handler: core.clone(),
            mouse_handler: core.clone(),
            mouse_sensitivity: 1.0,
        },
        MoveConfig {
            forward_speed: 2.0,
            backward_speed: 2.0,
            ..Default::default()
        },
    )
}
