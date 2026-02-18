use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::color::Color;
use crate::geometry::primitive::v3d::Vertex3D;
use crate::graphics::texture::t2d::Texture2DBuilder;
use crate::support::text::{text_2d_image, TextConfig, Typeface};
use crate::graphics::texture::RawImage;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};

static TC: TextConfig = TextConfig {
    foreground: Color::RED,
    background: Color::TRANSPARENT,
    typeface: Typeface::Generic,
};

pub(crate) fn show_cam_coords(
    g2d: &mut Graph2D,
    config: &EngineConfig,
    camera: &Camera,
    scale: f32,
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
    let height = 13.7 * scale;
    let x = 10.0;
    let y = 20.0;
    let y_cam = y;
    let y_forward = y + height;
    let y_right = y + height + height;
    let y_up = y + height + height + height;

    /* update models */
    g2d.attach_or_update("6-2d-text-cam-pos", || create_pos_model(x, y_cam, TC.clone(), &position), |m| m.textures[0].replacement = create_pos_text(TC.clone(), &position));
    g2d.attach_or_update("6-2d-text-forward", || create_forward_model(x, y_forward, TC.clone(), &forward), |m| m.textures[0].replacement = create_forward_text(TC.clone(), &forward));
    g2d.attach_or_update("6-2d-text-right", || create_right_model(x, y_right, TC.clone(), &right), |m| m.textures[0].replacement = create_right_text(TC.clone(), &right));
    g2d.attach_or_update("6-2d-text-up", || create_up_model(x, y_up, TC.clone(), &up), |m| m.textures[0].replacement = create_up_text(TC.clone(), &up));
}

fn create_pos_model(x: f32, y_cam: f32, config: TextConfig, position: &Vertex3D) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_cam)
            .with_image(create_pos_text(config, &position).unwrap())
            .build())
        .build()
}

fn create_pos_text(config: TextConfig, position: &Vertex3D) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "cam-pos: ({:+08.2},{:+08.2},{:+08.2})",
            position.x,
            position.y,
            position.z,
        ))
    }))
}

fn create_forward_model(x: f32, y_forward: f32, config: TextConfig, forward: &Vertex3D) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_forward)
            .with_image(create_forward_text(config, &forward).unwrap())
            .build())
        .build()
}

fn create_forward_text(config: TextConfig, forward: &Vertex3D) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "forward: ({:+.2},{:+.2},{:+.2})",
            forward.x,
            forward.y,
            forward.z,
        ))
    }))
}

fn create_right_model(x: f32, y_right: f32, config: TextConfig, right: &Vertex3D) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_right)
            .with_image(create_right_text(config, &right).unwrap())
            .build())
        .build()
}

fn create_right_text(config: TextConfig, right: &Vertex3D) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "right:   ({:+.2},{:+.2},{:+.2})",
            right.x,
            right.y,
            right.z,
        ))
    }))
}

fn create_up_model(x: f32, y_up: f32, config: TextConfig, up: &Vertex3D) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_up)
            .with_image(create_up_text(config, &up).unwrap())
            .build())
        .build()
}

fn create_up_text(config: TextConfig, up: &Vertex3D) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "up:      ({:+.2},{:+.2},{:+.2})",
            up.x,
            up.y,
            up.z,
        ))
    }))
}
