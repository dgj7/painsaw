use crate::config::EngineConfig;
use crate::geometry::primitive::v3d::Vertex3D;
use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};
use crate::graphics::texture::t2d::Texture2DBuilder;
use crate::support::image::RawImage;
use crate::support::stats::{HEIGHT, TC, X_POS};
use crate::support::text::{text_2d_image, TextConfig};

pub(crate) fn show_cam_coords(
    g2d: &mut Graph2D,
    config: &EngineConfig,
    camera: &Camera,
) {
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
    let y_right = y + HEIGHT + HEIGHT;
    let y_up = y + HEIGHT + HEIGHT + HEIGHT;

    /* update models */
    g2d.attach_or_update("99-2d-text-cam-pos", || create_model(X_POS, y_cam,     "cam-pos: ", TC.clone(), &position), |m| m.textures[0].replacement = create_text(TC.clone(), "cam-pos: ", &position));
    g2d.attach_or_update("99-2d-text-forward", || create_model(X_POS, y_forward, "forward: ", TC.clone(), &forward), |m| m.textures[0].replacement =  create_text(TC.clone(), "forward: ", &forward));
    g2d.attach_or_update("99-2d-text-right",   || create_model(X_POS, y_right,   "right:   ", TC.clone(), &right), |m| m.textures[0].replacement =    create_text(TC.clone(), "right:   ", &right));
    g2d.attach_or_update("99-2d-text-up",      || create_model(X_POS, y_up,      "up:      ", TC.clone(), &up), |m| m.textures[0].replacement =       create_text(TC.clone(), "up:      ", &up));

    // todo: print yaw(left/right, about y), pitch(up,down, about x), roll(side/side, about z)
}

fn create_model(x: f32, y_cam: f32, label: &str, config: TextConfig, vertex: &Vertex3D) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_cam)
            .with_image(create_text(config, label, &vertex).unwrap())
            .build())
        .build()
}

fn create_text(config: TextConfig, label: &str, position: &Vertex3D) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "{}({:+08.2},{:+08.2},{:+08.2})",
            label,
            position.x,
            position.y,
            position.z,
        ))
    }))
}
