use crate::config::EngineConfig;
use crate::graphics::color::Color;
use crate::graphics::image::t2d::Texture2DBuilder;
use crate::support::text::{text_2d_image, TextConfig, Typeface};
use crate::graphics::image::RawImage;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};
use crate::support::timing::EngineTiming;

static TC: TextConfig = TextConfig {
    foreground: Color::RED,
    background: Color::TRANSPARENT,
    typeface: Typeface::Generic,
};

pub(crate) fn show_fps(g2d: &mut Graph2D, timing: &EngineTiming, config: &EngineConfig) {
    /* nothing to do if not enabled */
    if !config.renderer.show_fps {
        return;
    }

    /* calculate fps */
    let fps = timing.compute_fps();
    let avg = timing.compute_avg_fps();

    /* add or update models */
    g2d.attach_or_update("99-builtin-fps", || create_model(TC.clone(), fps, avg), |m| m.textures[0].replacement = Option::from(create_text(TC.clone(), fps, avg)));
}

fn create_text(config: TextConfig, fps: u32, avg: u32) -> RawImage {
    text_2d_image(config.clone(), || String::from(format!("FPS:{:4} ({:4} avg)", fps, avg)))
}

fn create_model(config: TextConfig, fps: u32, avg: u32) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(10.0)
            .with_y(5.0)
            .with_image(create_text(config, fps, avg))
            .build())
        .build()
}
