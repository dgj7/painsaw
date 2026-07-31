use crate::config::EngineConfig;
use crate::geometry::primitive::v2d::Vertex2D;
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
    client_center: &Vertex2D,
    window_center: &Vertex2D,
    mouse_position: &Vertex2D,
) {
    /* nothing to do if not enabled */
    if !config.renderer.show_screen_stats {
        return;
    }

    /* positioning variables */
    let y = 80.0;
    let y_cr = y;
    let y_wr = y + HEIGHT;
    let y_cc = y + HEIGHT + HEIGHT;
    let y_wc = y + HEIGHT + HEIGHT + HEIGHT;
    let y_mp = y + HEIGHT + HEIGHT + HEIGHT + HEIGHT;

    /* update models */
    g2d.attach_or_update("99-2d-screen-client-rect", || create_rect_model(X_POS, y_cr, TC.clone(), &client_rect, "clt pos"), |m| m.textures[0].replacement = create_rect_text(TC.clone(), &client_rect, "clt pos"));
    g2d.attach_or_update("99-2d-screen-window-rect", || create_rect_model(X_POS, y_wr, TC.clone(), &window_rect, "win pos"), |m| m.textures[0].replacement = create_rect_text(TC.clone(), &window_rect, "win pos"));
    g2d.attach_or_update("99-2d-screen-client-center", || create_vertex_model(X_POS, y_cc, TC.clone(), &client_center, "clt ctr"), |m| m.textures[0].replacement = create_vertex_text(TC.clone(), &client_center, "clt ctr"));
    g2d.attach_or_update("99-2d-screen-window-center", || create_vertex_model(X_POS, y_wc, TC.clone(), &window_center, "win ctr"), |m| m.textures[0].replacement = create_vertex_text(TC.clone(), &window_center, "win ctr"));
    g2d.attach_or_update("99-2d-screen-mouse-pos", || create_vertex_model(X_POS, y_mp, TC.clone(), &mouse_position, "mouse  "), |m| m.textures[0].replacement = create_vertex_text(TC.clone(), &mouse_position, "mouse  "));
}

fn create_rect_model(x: f32, y: f32, config: TextConfig, rect: &Rectangle2D, label: &str) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y)
            .with_image(create_rect_text(config, &rect, label).unwrap())
            .build())
        .build()
}

fn create_rect_text(config: TextConfig, rect: &Rectangle2D, label: &str) -> Option<RawImage> {
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

fn create_vertex_model(x: f32, y: f32, config: TextConfig, pt: &Vertex2D, label: &str) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y)
            .with_image(create_vertex_text(config, pt, label).unwrap())
            .build())
        .build()
}

fn create_vertex_text(config: TextConfig, pt: &Vertex2D, label: &str) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "{}: ({:+08.2},{:+08.2})",
            label,
            pt.x,
            pt.y,
        ))
    }))
}
