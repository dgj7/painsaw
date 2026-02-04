use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::color::Color;
use crate::graphics::geometry::primitive::v3d::Vertex3D;
use crate::graphics::image::t2d::Texture2DBuilder;
use crate::graphics::image::text::{text_2d_image, TextConfig, Typeface};
use crate::graphics::image::RawImage;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};
use num_traits::Float;

static TC: TextConfig = TextConfig {
    foreground: Color::RED,
    background: Color::TRANSPARENT,
    typeface: Typeface::Generic,
};

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

    /* update models */
    g2d.attach_or_update("6-2d-text-cam-pos", || create_pos_model(x, y_cam, TC.clone(), &position), |m| m.textures[0].replacement = create_pos_text(TC.clone(), &position));
    g2d.attach_or_update("6-2d-text-forward", || create_forward_model(x, y_forward, TC.clone(), &forward), |m| m.textures[0].replacement = create_forward_text(TC.clone(), &forward));
    g2d.attach_or_update("6-2d-text-right", || create_right_model(x, y_right, TC.clone(), &right), |m| m.textures[0].replacement = create_right_text(TC.clone(), &right));
    g2d.attach_or_update("6-2d-text-up", || create_up_model(x, y_up, TC.clone(), &up), |m| m.textures[0].replacement = create_up_text(TC.clone(), &up));
}

fn create_pos_model<F: Float>(x: F, y_cam: F, config: TextConfig, position: &Vertex3D<F>) -> Model2D<F> {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_cam)
            .with_image(create_pos_text(config, &position).unwrap())
            .build())
        .build()
}

fn create_pos_text<F: Float>(config: TextConfig, position: &Vertex3D<F>) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "cam-pos: ({:+08.2},{:+08.2},{:+08.2})",
            position.x.to_f32().unwrap(),
            position.y.to_f32().unwrap(),
            position.z.to_f32().unwrap(),
        ))
    }))
}

fn create_forward_model<F: Float>(x: F, y_forward: F, config: TextConfig, forward: &Vertex3D<F>) -> Model2D<F> {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_forward)
            .with_image(create_forward_text(config, &forward).unwrap())
            .build())
        .build()
}

fn create_forward_text<F: Float>(config: TextConfig, forward: &Vertex3D<F>) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "forward: ({:+.2},{:+.2},{:+.2})",
            forward.x.to_f32().unwrap(),
            forward.y.to_f32().unwrap(),
            forward.z.to_f32().unwrap(),
        ))
    }))
}

fn create_right_model<F: Float>(x: F, y_right: F, config: TextConfig, right: &Vertex3D<F>) -> Model2D<F> {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_right)
            .with_image(create_right_text(config, &right).unwrap())
            .build())
        .build()
}

fn create_right_text<F: Float>(config: TextConfig, right: &Vertex3D<F>) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "right:   ({:+.2},{:+.2},{:+.2})",
            right.x.to_f32().unwrap(),
            right.y.to_f32().unwrap(),
            right.z.to_f32().unwrap(),
        ))
    }))
}

fn create_up_model<F: Float>(x: F, y_up: F, config: TextConfig, up: &Vertex3D<F>) -> Model2D<F> {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y_up)
            .with_image(create_up_text(config, &up).unwrap())
            .build())
        .build()
}

fn create_up_text<F: Float>(config: TextConfig, up: &Vertex3D<F>) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "up:      ({:+.2},{:+.2},{:+.2})",
            up.x.to_f32().unwrap(),
            up.y.to_f32().unwrap(),
            up.z.to_f32().unwrap(),
        ))
    }))
}
