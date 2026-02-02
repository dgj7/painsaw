use crate::config::EngineConfig;
use crate::graphics::color::Color;
use crate::graphics::image::t2d::Texture2DBuilder;
use crate::graphics::image::text::{text_2d_image, TextConfig};
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::m2d::Model2DBuilder;
use crate::graphics::timing::EngineTiming;
use num_traits::Float;

pub(crate) fn show_fps<F: Float>(g2d: &mut Graph2D<F>, timing: &EngineTiming, config: &EngineConfig<F>) {
    /* nothing to do if not enabled */
    if !config.renderer.show_fps {
        return;
    }

    /* calculate fps */
    let fps = timing.compute_fps();

    /* prepare to render text */
    let config = TextConfig {
        foreground: Color::RED,
        ..Default::default()
    };

    /* add or update models */
    g2d.attach_or_update(
        "99-builtin-fps",
        Model2DBuilder::new()
            .with_texture(
                Texture2DBuilder::new()
                    .with_x(F::from(10.0).unwrap())
                    .with_y(F::from(5.0).unwrap())
                    .with_image(text_2d_image(config.clone(), || {
                        String::from(format!("FPS:{:4}", fps))
                    }))
                    .build(),
            )
            .build(),
        |m| {
            m.textures[0].replacement = Option::from(text_2d_image(config.clone(), || {
                String::from(format!("FPS:{:4}", fps))
            }))
        },
    );
}
