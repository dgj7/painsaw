use crate::config::EngineConfig;
use crate::geometry::rect::Rectangle2D;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};
use crate::graphics::texture::t2d::Texture2DBuilder;
use crate::support::image::RawImage;
use crate::support::stats::{HEIGHT, TC, X_POS};
use crate::support::text::{text_2d_image, TextConfig};

///
/// display various screen statistics.
///
/// * window rect
/// * client rect
/// * center of screen, from operating system perspective
///
pub(crate) fn show_screen_stats(
    g2d: &mut Graph2D,
    config: &EngineConfig,
    client_rect: &Rectangle2D,
    window_rect: &Rectangle2D,
) {
    /* nothing to do if not enabled */
    if !config.renderer.show_screen_stats {
        return;
    }

    /* positioning variables */
    let y = 80.0;
    let y_client = y;
    let y_window = y + HEIGHT;

    /* update models */
    g2d.attach_or_update("99-2d-screen-client", || create_model(X_POS, y_client, TC.clone(), &client_rect, "client"), |m| m.textures[0].replacement = create_text(TC.clone(), &client_rect, "client"));
    g2d.attach_or_update("99-2d-screen-window", || create_model(X_POS, y_window, TC.clone(), &window_rect, "window"), |m| m.textures[0].replacement = create_text(TC.clone(), &window_rect, "window"));
}

fn create_model(x: f32, y: f32, config: TextConfig, rect: &Rectangle2D, label: &str) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y)
            .with_image(create_text(config, &rect, label).unwrap())
            .build())
        .build()
}

fn create_text(config: TextConfig, rect: &Rectangle2D, label: &str) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "{}: ({:+08.2},{:+08.2}),({:+08.2},{:+08.2})",
            label,
            rect.top_left.x,
            rect.top_left.y,
            rect.bottom_right.x,
            rect.bottom_right.y,
        ))
    }))
}
