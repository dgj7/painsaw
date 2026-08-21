use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::v3d::Vertex3D;
use crate::geometry::rect::Rectangle2D;
use crate::graphics::color::Color;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};
use crate::graphics::texture::t2d::Texture2DBuilder;
use crate::support::image::RawImage;
use crate::support::text::{text_2d_image, TextConfig, Typeface};

pub mod coords;
pub mod fps;
pub mod screen;

pub(crate) static TC: TextConfig = TextConfig {
    foreground: Color::RED,
    background: Color::TRANSPARENT,
    typeface: Typeface::Generic,
};

pub(crate) static HEIGHT: f32 = 13.7;
pub(crate) static X_POS: f32 = 10.0;

pub(crate) fn create_vertex2d_model(x: f32, y: f32, config: TextConfig, label: &str, pt: &Vertex2D) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y)
            .with_image(create_vertex2d_text(config, label, pt).unwrap())
            .build())
        .build()
}

pub(crate) fn create_vertex2d_text(config: TextConfig, label: &str, pt: &Vertex2D) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "{}({:+08.2},{:+08.2})",
            label,
            pt.x,
            pt.y,
        ))
    }))
}

pub(crate) fn create_vertex3d_model(x: f32, y: f32, label: &str, config: TextConfig, target: &Vertex3D) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y)
            .with_image(create_vertex3d_text(config, label, &target).unwrap())
            .build())
        .build()
}

pub(crate) fn create_vertex3d_text(config: TextConfig, label: &str, target: &Vertex3D) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "{}({:+08.2},{:+08.2},{:+08.2})",
            label,
            target.x,
            target.y,
            target.z,
        ))
    }))
}

pub(crate) fn create_f32_model(x: f32, y: f32, config: TextConfig, label: &str, value: f32) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y)
            .with_image(create_f32_text(config, label, value).unwrap())
            .build())
        .build()
}

pub(crate) fn create_f32_text(config: TextConfig, label: &str, value: f32) -> Option<RawImage> {
    Option::from(text_2d_image(config.clone(), || {
        String::from(format!(
            "{}{:+08.2}",
            label,
            value
        ))
    }))
}

pub(crate) fn create_rect2d_model(x: f32, y: f32, config: TextConfig, label: &str, rect: &Rectangle2D) -> Model2D {
    Model2DBuilder::new()
        .with_texture(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y)
            .with_image(create_rect2d_text(config, label, &rect).unwrap())
            .build())
        .build()
}

pub(crate) fn create_rect2d_text(config: TextConfig, label: &str, rect: &Rectangle2D) -> Option<RawImage> {
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
