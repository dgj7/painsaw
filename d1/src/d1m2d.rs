use engine::geometry::primitive::prim2d::Primitive2DBuilder;
use engine::geometry::primitive::v2d::Vertex2D;
use engine::geometry::primitive::PrimitiveType;
use engine::graphics::camera::Camera;
use engine::graphics::color::Color;
use engine::graphics::storage::m2d::{Model2D, Model2DBuilder};
use std::f32::consts::PI;

pub(super) fn create_2d_axes(camera: &Camera) -> Model2D {
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 10.0})
            .with_color(Color::from_rgba(0.498, 0.0, 1.0, 1.0))
            .with_vertex(Vertex2D::origin())
            .with_vertex(Vertex2D::new(0.0, camera.projection.height))
            .with_vertex(Vertex2D::origin())
            .with_vertex(Vertex2D::new(camera.projection.width, 0.0))
            .build())
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Point {point_size: 15.0})
            .with_color(Color::GREEN)
            .with_vertex(Vertex2D::origin())
            .with_vertex(Vertex2D::new(0.0, camera.projection.height))
            .with_vertex(Vertex2D::new(camera.projection.width, 0.0))
            .build())
        .build()
}

pub(super) fn create_2d_grid_x_lines(camera: &Camera) -> Model2D {
    /* storage for vertices */
    let mut vertices = vec!();

    /* define line vertices */
    let hgap = 10;
    let hiters = ((camera.projection.height + (hgap as f32))/(hgap as f32)) as u16;
    for h in 0..hiters {
        vertices.push(Vertex2D::new(0.0, (h * hgap) as f32));
        vertices.push(Vertex2D::new(camera.projection.width, (h * hgap) as f32));
    }

    /* done */
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::from_rgba(0.2, 0.2, 0.2, 0.5))
            .with_vertices(vertices)
            .build())
        .build()
}

pub(super) fn create_2d_grid_y_lines(camera: &Camera) -> Model2D {
    /* storage for vertices */
    let mut vertices = vec!();

    /* define line vertices */
    let vgap = 10;
    let viters = ((camera.projection.width + (vgap as f32))/(vgap as f32)) as u16;
    for v in 0..viters {
        vertices.push(Vertex2D::new((v * vgap) as f32, 0.0));
        vertices.push(Vertex2D::new((v * vgap) as f32, camera.projection.height));
    }

    /* done */
    Model2DBuilder::new()
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::from_rgba(0.2, 0.2, 0.2, 1.0))
            .with_vertices(vertices)
            .build())
        .build()
}

pub(super) fn create_2d_crosshairs(camera: &Camera) -> Model2D {
    /* centering values */
    let center_x = camera.projection.width / 2.0;
    let center_y = camera.projection.height / 2.0;

    /* crosshair values */
    let crosshair_len = 20.0;

    /* circle values */
    let mut circle_vertices = vec!();
    let radius = crosshair_len + 5.0;
    let count = 360;
    for idx in 0..count {
        let angle = (idx as f32 / count as f32) * 2.0 * PI;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        circle_vertices.push(Vertex2D::new(x, y));
    }

    /* hash marks; 4 about circle */
    let hmlen = 10.0;


    /* create crosshair model */
    Model2DBuilder::new()
        /* draw white cross */
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.99))
            .with_vertex(Vertex2D::new(center_x - crosshair_len / 2.0, center_y))
            .with_vertex(Vertex2D::new(center_x + crosshair_len / 2.0, center_y))
            .with_vertex(Vertex2D::new(center_x, center_y - crosshair_len / 2.0))
            .with_vertex(Vertex2D::new(center_x, center_y + crosshair_len / 2.0))
            .build())
        /* draw circle */
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::LineStrip {thickness: 1.0})
            .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.99))
            .with_vertices(circle_vertices)
            .build())
        /* draw hash marks */
        .with_primitive(Primitive2DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.99))
            .with_vertex(Vertex2D::new(center_x, center_y - radius - hmlen / 2.0))
            .with_vertex(Vertex2D::new(center_x, center_y - radius + hmlen / 2.0))
            .with_vertex(Vertex2D::new(center_x, center_y + radius - hmlen / 2.0))
            .with_vertex(Vertex2D::new(center_x, center_y + radius + hmlen / 2.0))
            .with_vertex(Vertex2D::new(center_x - radius - hmlen / 2.0, center_y))
            .with_vertex(Vertex2D::new(center_x - radius + hmlen / 2.0, center_y))
            .with_vertex(Vertex2D::new(center_x + radius - hmlen / 2.0, center_y))
            .with_vertex(Vertex2D::new(center_x + radius + hmlen / 2.0, center_y))
            .build())
        .build()
}
