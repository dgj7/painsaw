use engine::input::model::keyboard_state::{KeyInfo, KeyPosition};
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::math::twod::draw_config_2d::DrawingConfig2D;
use engine::math::twod::line_2d::Line2D;
use engine::math::twod::point_2d::Point2D;
use engine::render::graphics::opengl::opengl_wrapper::{paint_background, paint_lines, prepare_2d};
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

    fn before_render(&self, _context: &mut RendererContext) {
        // no work to do yet
    }

    fn render_2d_scene(&self, context: &mut RendererContext) {
        context.first_frame_rendered = true;
        context.frame_count += 1;

        /* gather needed data */
        let ccd = context.copy_client_dimensions();

        /* prepare for 2d drawing */
        prepare_2d(context);

        /* draw black background */
        paint_background(Color::BLACK);

        /* draw x&y axes in white */
        paint_lines(&*vec!(
            Line2D::new(Point2D::origin(), Point2D::new(0.0, ccd.height)),
            Line2D::new(Point2D::origin(), Point2D::new(ccd.width, 0.0)),
        ), &DrawingConfig2D::new(Color::from_rgb(0.498, 0.0, 1.0), 10.0));

        /* draw x-axis horizontal lines */
        let hgap = 10;
        let hiters = ((ccd.height + (hgap as f32))/(hgap as f32)) as u16;
        let mut hlines: Vec<Line2D> = Vec::with_capacity((hiters + 10) as usize);
        for h in 0..hiters {
            hlines.push(Line2D::new(Point2D::new(0.0, (h * hgap) as f32), Point2D::new(ccd.width, (h * hgap) as f32)));
        }
        paint_lines(&*hlines, &DrawingConfig2D::new(Color::from_rgb(0.2, 0.2, 0.2), 1.0));

        /* draw y-axis vertical lines */
        let vgap = 10;
        let viters = ((ccd.width + (vgap as f32))/(vgap as f32)) as u16;
        let mut vlines: Vec<Line2D> = Vec::with_capacity((viters + 10) as usize);
        for v in 0..viters {
            vlines.push(Line2D::new(Point2D::new((v * vgap) as f32, 0.0), Point2D::new((v * vgap) as f32, ccd.height)));
        }
        paint_lines(&*vlines, &DrawingConfig2D::new(Color::from_rgb(0.2, 0.2, 0.2), 1.0));
    }

    fn render_3d_scene(&self, _context: &mut RendererContext) {
        // no work to do yet
    }

    fn after_render(&self, _context: &mut RendererContext) {
        // no work to do yet
    }
}

impl GameRenderer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
