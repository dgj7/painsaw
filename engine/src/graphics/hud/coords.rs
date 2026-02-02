use num_traits::Float;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::color::Color;
use crate::graphics::geometry::primitive::v3d::Vertex3D;
use crate::graphics::image::RawImage;
use crate::graphics::image::t2d::Texture2DBuilder;
use crate::graphics::image::text::{text_2d_image, TextConfig};
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::m2d::Model2DBuilder;

pub(crate) fn show_cam_coords<F: Float>(
    g2d: &mut Graph2D<F>,
    config: &EngineConfig<F>,
    camera: &Camera<F>,
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
    let x = F::from(10.0).unwrap();
    let y = 20.0;
    let y_cam = F::from(y).unwrap();
    let y_forward = F::from(y + height).unwrap();
    let y_right = F::from(y + height + height).unwrap();
    let y_up = F::from(y + height + height + height).unwrap();

    /* determine how the text should be displayed */
    let config = TextConfig {
        foreground: Color::RED,
        ..Default::default()
    };

    /* update models */
    g2d.attach_or_update(
        "6-2d-text-cam-pos",
        Model2DBuilder::new()
            .with_texture(
                Texture2DBuilder::new()
                    .with_x(x)
                    .with_y(y_cam)
                    .with_image(create_text_cam_pos(config.clone(), &position).unwrap())
                    .build(),
            )
            .build(),
        |m| m.textures[0].replacement = create_text_cam_pos(config.clone(), &position),
    );
    g2d.attach_or_update(
        "6-2d-text-forward",
        Model2DBuilder::new()
            .with_texture(
                Texture2DBuilder::new()
                    .with_x(x)
                    .with_y(y_forward)
                    .with_image(create_text_forward(config.clone(), &forward).unwrap())
                    .build(),
            )
            .build(),
        |m| m.textures[0].replacement = create_text_forward(config.clone(), &forward),
    );
    g2d.attach_or_update(
        "6-2d-text-right",
        Model2DBuilder::new()
            .with_texture(
                Texture2DBuilder::new()
                    .with_x(x)
                    .with_y(y_right)
                    .with_image(create_text_right(config.clone(), &right).unwrap())
                    .build(),
            )
            .build(),
        |m| m.textures[0].replacement = create_text_right(config.clone(), &right),
    );
    g2d.attach_or_update(
        "6-2d-text-up",
        Model2DBuilder::new()
            .with_texture(
                Texture2DBuilder::new()
                    .with_x(x)
                    .with_y(y_up)
                    .with_image(create_text_up(config.clone(), &up).unwrap())
                    .build(),
            )
            .build(),
        |m| m.textures[0].replacement = create_text_up(config.clone(), &up),
    );
}

fn create_text_cam_pos<F: Float>(config: TextConfig, position: &Vertex3D<F>) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "cam-pos: ({:+08.2},{:+08.2},{:+08.2})",
            position.x.to_f32().unwrap(),
            position.y.to_f32().unwrap(),
            position.z.to_f32().unwrap(),
        ))
    }))
}

fn create_text_up<F: Float>(config: TextConfig, up: &Vertex3D<F>) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "up:      ({:+.2},{:+.2},{:+.2})",
            up.x.to_f32().unwrap(),
            up.y.to_f32().unwrap(),
            up.z.to_f32().unwrap(),
        ))
    }))
}

fn create_text_right<F: Float>(config: TextConfig, right: &Vertex3D<F>) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "right:   ({:+.2},{:+.2},{:+.2})",
            right.x.to_f32().unwrap(),
            right.y.to_f32().unwrap(),
            right.z.to_f32().unwrap(),
        ))
    }))
}

fn create_text_forward<F: Float>(config: TextConfig, forward: &Vertex3D<F>) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "forward: ({:+.2},{:+.2},{:+.2})",
            forward.x.to_f32().unwrap(),
            forward.y.to_f32().unwrap(),
            forward.z.to_f32().unwrap(),
        ))
    }))
}
