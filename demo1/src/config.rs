use engine::config::EngineConfig;
use engine::config::input_config::InputConfig;
use engine::config::renderer_config::RendererConfig;
use engine::config::window_config::{WindowConfig, WindowDimensions};
use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};
use engine::input::kn::KeyName;
use engine::input::ks::KeyState;
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
            fps_cap: Some(999),
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

fn create_behaviors() -> HashMap<KeyName, fn(&mut RendererContext<f32>, &KeyState)> {
    let mut behaviors: HashMap<KeyName, fn(&mut RendererContext<f32>, &KeyState)> = HashMap::new();

    behaviors.insert(KeyName::KeyG, handle_g);

    behaviors
}

fn handle_g(_context: &mut RendererContext<f32>, state: &KeyState) {
    logging_key_behavior(KeyName::KeyG, state);
}

fn logging_key_behavior(name: KeyName, state: &KeyState) {
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
