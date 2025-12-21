use engine::input::model::keyboard_state::{KeyInfo, KeyPosition};
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::render::graphics::opengl::opengl_wrapper::paint_background;
use engine::render::model::color::Color;
use engine::render::model::render_context::RendererContext;
use engine::render::renderer::Renderer;

pub(crate) struct GameRenderer {}

impl Renderer for GameRenderer {
    fn update_world(&self, context: &mut RendererContext) {
        match context.input.clone().lock() {
            Ok(mut input) => {
                let duration = input.g_key.previous_key_state_duration();
                match &input.g_key.current {
                    KeyPosition::KeyDown { info } => {
                        if !info.handled {
                            log(LogLevel::Debug, &|| String::from(format!("G: DOWN    (up for {}ms)", duration.as_millis())));
                            input.g_key.current = KeyPosition::KeyDown { info: KeyInfo { when: info.when, handled: true, } };
                        }
                    }
                    KeyPosition::KeyUp { info } => {
                        if !info.handled {
                            log(LogLevel::Debug, &|| String::from(format!("G: UP      (down for {}ms)", duration.as_millis())));
                            input.g_key.current = KeyPosition::KeyUp { info: KeyInfo { when: info.when, handled: true } };
                        }
                    }
                }
            },
            Err(_) => {
                panic!("todo: handle mutex lock failure")
            }
        }
    }

    fn render_scene(&self, context: &mut RendererContext) {
        context.first_frame_rendered = true;
        context.frame_count += 1;

        paint_background(Color::BLACK);
    }
}

impl GameRenderer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
