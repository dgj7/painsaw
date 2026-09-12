use engine::config::EngineConfig;
use engine::config::input_config::InputConfig;
use engine::config::move_config::MoveConfig;
use engine::config::renderer_config::RendererConfig;
use engine::config::window_config::{WindowConfig, WindowDimensions};
use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};

pub fn create_engine_config() -> EngineConfig {
    EngineConfig::new(
        WindowConfig {
            dimensions: WindowDimensions::Dimensional { width: 1920, height: 1080, },
            title: Some(String::from("Demo3 - MsWin/OpenGL - User Interfaces")),
            window_id: Some(String::from("PAINSAW-DEMO3")),
        },
        RendererConfig {
            subsystem: GraphicsSubSystem::OpenGL { pipeline: OpenGLPipeline::FixedFunction, },
            show_fps: true,
            show_cam_coords: false,
            show_screen_stats: false,
            fps_cap: Some(240),
        },
        InputConfig {
            mouse_sensitivity: 1.0,
        },
        MoveConfig {
            forward_speed: 2.0,
            backward_speed: 2.0,
            ..Default::default()
        },
    )
}
