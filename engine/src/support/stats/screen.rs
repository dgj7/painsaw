use crate::config::EngineConfig;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::support::stats::{
    create_rect2d_model, create_rect2d_text, create_vertex2d_model, create_vertex2d_text, HEIGHT, TC,
    X_POS,
};

static CLT_POS: &str = "clt pos: ";
static WIN_POS: &str = "win pos: ";
static CLT_CTR: &str = "clt ctr: ";
static WIN_CTR: &str = "win ctr: ";
static MOS_POS: &str = "mouse:   ";

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
    camera: &Camera,
    mouse_position: &Vertex2D,
) {
    /* nothing to do if not enabled */
    if !config.renderer.show_screen_stats {
        return;
    }

    /* gather variables */
    let client_rect = &camera.screen.current_client_rect;
    let window_rect = &camera.screen.current_window_rect;
    let client_center = &camera.screen.client_center;
    let window_center = &camera.screen.window_center;

    /* positioning variables */
    let y = 120.0;
    let y_cr = y;
    let y_wr = y + HEIGHT;
    let y_cc = y + HEIGHT + HEIGHT;
    let y_wc = y + HEIGHT + HEIGHT + HEIGHT;
    let y_mp = y + HEIGHT + HEIGHT + HEIGHT + HEIGHT;

    /* update models */
    g2d.attach_or_update(
        "99-2d-screen-client-rect",
        || create_rect2d_model(X_POS, y_cr, TC.clone(), CLT_POS, &client_rect),
        |m| m.textures[0].replacement = create_rect2d_text(TC.clone(), CLT_POS, &client_rect),
    );
    g2d.attach_or_update(
        "99-2d-screen-window-rect",
        || create_rect2d_model(X_POS, y_wr, TC.clone(), WIN_POS, &window_rect),
        |m| m.textures[0].replacement = create_rect2d_text(TC.clone(), WIN_POS, &window_rect),
    );
    g2d.attach_or_update(
        "99-2d-screen-client-center",
        || create_vertex2d_model(X_POS, y_cc, TC.clone(), CLT_CTR, &client_center),
        |m| m.textures[0].replacement = create_vertex2d_text(TC.clone(), CLT_CTR, &client_center),
    );
    g2d.attach_or_update(
        "99-2d-screen-window-center",
        || create_vertex2d_model(X_POS, y_wc, TC.clone(), WIN_CTR, &window_center),
        |m| m.textures[0].replacement = create_vertex2d_text(TC.clone(), WIN_CTR, &window_center),
    );
    g2d.attach_or_update(
        "99-2d-screen-mouse-pos",
        || create_vertex2d_model(X_POS, y_mp, TC.clone(), MOS_POS, &mouse_position),
        |m| m.textures[0].replacement = create_vertex2d_text(TC.clone(), MOS_POS, &mouse_position),
    );
}
