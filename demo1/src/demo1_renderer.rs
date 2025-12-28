use engine::input::model::keyboard_state::{KeyInfo, KeyPosition};
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::render::graphics::opengl::opengl_api::gl_viewport;
use engine::render::graphics::opengl::opengl_operations::{paint_axes, paint_grid};
use engine::render::graphics::opengl::opengl_wrapper_2d::{paint_background, prepare_2d};
use engine::render::graphics::opengl::opengl_wrapper_3d::prepare_3d;
use engine::render::model::color::Color;
use engine::render::model::render_context::RendererContext;
use engine::render::renderer::Renderer;

pub(crate) struct Demo1Renderer {}

impl Renderer for Demo1Renderer {
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

    fn before_render(&self, context: &mut RendererContext) {
        paint_background(Color::BLACK);

        let ccd = context.copy_client_dimensions();
        gl_viewport(0, 0, ccd.width as i32, ccd.height as i32);
    }

    fn render_2d_scene(&self, context: &mut RendererContext) {
        /* gather needed data */
        let ccd = context.copy_client_dimensions();

        /* prepare for 2d drawing */
        prepare_2d(context);

        /* paint the full-screen grid */
        paint_grid(&ccd);
    }

    fn render_3d_scene(&self, context: &mut RendererContext) {
        /* prepare for 3d drawing */
        prepare_3d(context);

        /* paint the axes */
        paint_axes();
    }

    fn after_render(&self, context: &mut RendererContext) {
        context.first_frame_rendered = true;
        context.frame_count += 1;
    }
}

impl Demo1Renderer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
