use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};
use engine::window::window_config::{WindowConfig, WindowDimensions};

pub fn create_window_config() -> WindowConfig {
    WindowConfig {
        dimensions: WindowDimensions::Dimensional { width: 1920, height: 1080 },
        title: Some(String::from("Demo1 - MsWin/OpenGL")),
        wndclass: Some(String::from("PAINSAW-DEMO1")),
        graphics: GraphicsSubSystem::OpenGL { pipeline: OpenGLPipeline::FixedFunction },
    }
}
