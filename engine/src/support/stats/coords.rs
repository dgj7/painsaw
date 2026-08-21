use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::support::stats::{
    create_f32_model, create_f32_text, create_vertex3d_model, create_vertex3d_text, HEIGHT, TC,
    X_POS,
};

static CAM_POS: &str = "cam pos: ";
static FORWARD: &str = "forward: ";
static RIGHT: &str = "right:   ";
static UP: &str = "up:      ";
static PITCH: &str = "pitch:   ";
static YAW: &str = "yaw:     ";
static ROLL: &str = "roll:    ";

pub(crate) fn show_cam_coords(g2d: &mut Graph2D, config: &EngineConfig, camera: &Camera) {
    /* nothing to do if not enabled */
    if !config.renderer.show_cam_coords {
        return;
    }

    /* get all the camera info */
    let position = camera.orientation.position.column_major_position();
    let forward = camera.orientation.position.column_major_z_forward();
    let right = camera.orientation.position.column_major_x_right();
    let up = camera.orientation.position.column_major_y_up();

    /* positioning variables */
    let y = 20.0;
    let y_cam = y;
    let y_forward = y + HEIGHT;
    let y_right = y + HEIGHT * 2.0;
    let y_up = y + HEIGHT * 3.0;
    let y_pitch = y + HEIGHT * 4.0;
    let y_yaw = y + HEIGHT * 5.0;
    let y_roll = y + HEIGHT * 6.0;

    /* update models */
    g2d.attach_or_update(
        "99-2d-text-cam-pos",
        || create_vertex3d_model(X_POS, y_cam, CAM_POS, TC.clone(), &position),
        |m| m.textures[0].replacement = create_vertex3d_text(TC.clone(), CAM_POS, &position),
    );
    g2d.attach_or_update(
        "99-2d-text-forward",
        || create_vertex3d_model(X_POS, y_forward, FORWARD, TC.clone(), &forward),
        |m| m.textures[0].replacement = create_vertex3d_text(TC.clone(), FORWARD, &forward),
    );
    g2d.attach_or_update(
        "99-2d-text-right",
        || create_vertex3d_model(X_POS, y_right, RIGHT, TC.clone(), &right),
        |m| m.textures[0].replacement = create_vertex3d_text(TC.clone(), RIGHT, &right),
    );
    g2d.attach_or_update(
        "99-2d-text-up",
        || create_vertex3d_model(X_POS, y_up, UP, TC.clone(), &up),
        |m| m.textures[0].replacement = create_vertex3d_text(TC.clone(), UP, &up),
    );
    g2d.attach_or_update(
        "99-2d-pitch",
        || create_f32_model(X_POS, y_pitch, TC.clone(), PITCH, camera.orientation.pitch),
        |m| {
            m.textures[0].replacement = create_f32_text(TC.clone(), PITCH, camera.orientation.pitch)
        },
    );
    g2d.attach_or_update(
        "99-2d-yaw",
        || create_f32_model(X_POS, y_yaw, TC.clone(), YAW, camera.orientation.yaw),
        |m| m.textures[0].replacement = create_f32_text(TC.clone(), YAW, camera.orientation.yaw),
    );
    g2d.attach_or_update(
        "99-2d-roll",
        || create_f32_model(X_POS, y_roll, TC.clone(), ROLL, camera.orientation.roll),
        |m| m.textures[0].replacement = create_f32_text(TC.clone(), ROLL, camera.orientation.roll),
    );
}
