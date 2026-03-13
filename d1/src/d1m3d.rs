use engine::graphics::color::Color;
use engine::geometry::build::quad::QuadBuilder;
use engine::geometry::orient::matrix::m4x4::Matrix4x4;
use engine::geometry::orient::Orientation;
use engine::geometry::primitive::prim3d::Primitive3DBuilder;
use engine::geometry::primitive::PrimitiveType;
use engine::geometry::primitive::v3d::Vertex3D;
use engine::graphics::storage::m3d::{Model3D, Model3DBuilder};

pub(super) fn create_3d_axes() -> Model3D {
    Model3DBuilder::new()
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Point {point_size: 5.0})
            .with_color(Color::WHITE)
            .with_vertex(Vertex3D::new(0.0, 0.0, 0.0))
            .build())
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::RED)
            .with_vertex(Vertex3D::origin())
            .with_vertex(Vertex3D::new(0.5, 0.0, 0.0))
            .build())
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Point {point_size: 5.0})
            .with_color(Color::WHITE)
            .with_vertex(Vertex3D::new(0.5, 0.0, 0.0))
            .build())
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::GREEN)
            .with_vertex(Vertex3D::origin())
            .with_vertex(Vertex3D::new(0.0, 0.5, 0.0))
            .build())
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Point {point_size: 5.0})
            .with_color(Color::WHITE)
            .with_vertex(Vertex3D::new(0.0, 0.5, 0.0))
            .build())
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Line {thickness: 1.0})
            .with_color(Color::BLUE)
            .with_vertex(Vertex3D::origin())
            .with_vertex(Vertex3D::new(0.0, 0.0, 0.5))
            .build())
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Point {point_size: 5.0})
            .with_color(Color::WHITE)
            .with_vertex(Vertex3D::new(0.0, 0.0, 0.5))
            .build())
        .build()
}

pub(super) fn create_3d_cuboid_1() -> Model3D {
    let orientation = Orientation::new(Matrix4x4::from(
        Vertex3D::origin(),
        Vertex3D::origin(),
        Vertex3D::origin(),
        Vertex3D::new(0.75, 0.5, -1.0),
    ), 1.0, 1.0, 1.0);
    Model3DBuilder::new()
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Point{point_size: 5.0})
            .with_orientation(orientation.clone())
            .with_color(Color::WHITE)
            .with_vertex(Vertex3D::origin())
            .build())
        .with_primitive(QuadBuilder::new()
            .with_orientation(orientation)
            .with_width(0.25)
            .with_height(0.25)
            .with_depth(3.0)
            .with_color(Color::YELLOW)
            .build()
            .expect("invalid quad"))
        .build()
}

pub(super) fn create_3d_cuboid_wall_2() -> Model3D {
    let orientation = Orientation::new(Matrix4x4::from(
        Vertex3D::origin(),
        Vertex3D::origin(),
        Vertex3D::origin(),
        Vertex3D::new(-0.25, 0.5, -1.0)
    ), 1.0, 1.0, 1.0);
    Model3DBuilder::new()
        .with_primitive(Primitive3DBuilder::new()
            .with_type(PrimitiveType::Point {point_size: 5.0})
            .with_orientation(orientation.clone())
            .with_color(Color::WHITE)
            .with_vertex(Vertex3D::origin())
            .build())
        .with_primitive(QuadBuilder::new()
            .with_orientation(orientation)
            .with_width(1.0)
            .with_height(1.0)
            .with_depth(0.25)
            .with_color(Color::YELLOW)
            .build()
            .expect("invalid quad"))
        .build()
}
