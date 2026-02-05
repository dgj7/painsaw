use engine::config::EngineConfig;
use engine::config::input_config::InputConfig;
use engine::config::renderer_config::RendererConfig;
use engine::config::window_config::{WindowConfig, WindowDimensions};
use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};
use engine::input::r#in::InputName;
use engine::input::is::InputState;
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::window::context::RendererContext;
use std::collections::HashMap;
use engine::config::move_config::MoveConfig;

pub fn create_engine_config() -> EngineConfig<f32> {
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
            behaviors: create_behaviors(),
        },
        MoveConfig {
            forward_speed: 2.0,
            backward_speed: 2.0,
            ..Default::default()
        },
    )
}

fn create_behaviors() -> HashMap<InputName, fn(&mut RendererContext<f32>, &InputState)> {
    let mut behaviors: HashMap<InputName, fn(&mut RendererContext<f32>, &InputState)> = HashMap::new();

    behaviors.insert(InputName::KeyG, handle_g);

    behaviors
}

fn handle_g(_context: &mut RendererContext<f32>, state: &InputState) {
    logging_key_behavior(InputName::KeyG, state);
}

fn logging_key_behavior(name: InputName, state: &InputState) {
    let duration = state.previous_key_state_duration();
    log(LogLevel::Debug, &|| {
        String::from(format!(
            "{}: {}    ({} for {}ms)",
            name,
            state.current,
            state.previous,
            duration.as_millis()
        ))
    });
}
