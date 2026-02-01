use engine::graphics::camera::Camera;
use engine::graphics::color::Color;
use engine::graphics::geometry::primitive::prim2d::Primitive2DBuilder;
use engine::graphics::geometry::primitive::PrimitiveType;
use engine::graphics::geometry::primitive::v2d::Vertex2D;
use engine::graphics::image::t2d::Texture2DBuilder;
use engine::graphics::image::text::{text_2d_image, TextConfig};
use engine::graphics::storage::m2d::{Model2D, Model2DBuilder};

pub(super) fn create_2d_axes(camera: &Camera<f32>) -> Model2D<f32> {
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 10.0})
            .with_color(Color::from_rgb(0.498, 0.0, 1.0))
            .with_vertex(Vertex2D::origin())
            .with_vertex(Vertex2D::new(0.0, camera.height))
            .with_vertex(Vertex2D::origin())
            .with_vertex(Vertex2D::new(camera.width, 0.0))
            .build())
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Point {point_size: 15.0})
            .with_color(Color::GREEN)
            .with_vertex(Vertex2D::origin())
            .with_vertex(Vertex2D::new(0.0, camera.height))
            .with_vertex(Vertex2D::new(camera.width, 0.0))
            .build())
        .build()
}

pub(super) fn create_2d_grid_x_lines(camera: &Camera<f32>) -> Model2D<f32> {
    /* storage for vertices */
    let mut vertices = vec!();

    /* define line vertices */
    let hgap = 10;
    let hiters = ((camera.height + (hgap as f32))/(hgap as f32)) as u16;
    for h in 0..hiters {
        vertices.push(Vertex2D::new(0.0, (h * hgap) as f32));
        vertices.push(Vertex2D::new(camera.width, (h * hgap) as f32));
    }

    /* done */
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::from_rgb(0.2, 0.2, 0.2))
            .with_vertices(vertices)
            .build())
        .build()
}

pub(super) fn create_2d_grid_y_lines(camera: &Camera<f32>) -> Model2D<f32> {
    /* storage for vertices */
    let mut vertices = vec!();

    /* define line vertices */
    let vgap = 10;
    let viters = ((camera.width + (vgap as f32))/(vgap as f32)) as u16;
    for v in 0..viters {
        vertices.push(Vertex2D::new((v * vgap) as f32, 0.0));
        vertices.push(Vertex2D::new((v * vgap) as f32, camera.height));
    }

    /* done */
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::from_rgb(0.2, 0.2, 0.2))
            .with_vertices(vertices)
            .build())
        .build()
}

pub(super) fn create_2d_repeated_texts(count: u16, x: f32, y: f32) -> Model2D<f32> {
    let mut textures = vec!();

    for i in 0..count {
        textures.push(Texture2DBuilder::new()
            .with_x(x)
            .with_y(y + 20.0 * i as f32)
            .with_image(text_2d_image(TextConfig::default(), || "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string()))
            .build());
    }

    Model2D::new(vec!(), textures)
}
