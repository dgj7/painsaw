//!
//! create game world objects.
//!

use engine::geometry::orient::matrix::m4x4::Matrix4x4;
use engine::geometry::orient::OrientationBuilder;
use engine::geometry::primitive::v3d::Vertex3D;
use engine::graphics::storage::o3d::Object3D;

pub(super) fn create_object_triangle1(asset_id: usize) -> Object3D {
    Object3D {
        id: 5000,
        name: Some(String::from("triangle-1")),

        transform: OrientationBuilder::new()
            .with_transform(Matrix4x4::from(
                Vertex3D::create_x_unit(),
                Vertex3D::create_y_unit(),
                Vertex3D::create_z_unit(),
                Vertex3D::new(20.0, 0.0, 0.0),
            ))
            .build(),

        asset_id,

        visible: true,
        clipping: false,
    }
}

pub(super) fn create_object_quad1(asset_id: usize) -> Object3D {
    Object3D {
        id: 5001,
        name: Some(String::from("quad-1")),

        transform: OrientationBuilder::new()
            .with_transform(Matrix4x4::from(
                Vertex3D::create_x_unit(),
                Vertex3D::create_y_unit(),
                Vertex3D::create_z_unit(),
                Vertex3D::new(-20.0, 0.0, 0.0),
            ))
            .build(),

        asset_id,

        visible: true,
        clipping: false,
    }
}

pub(super) fn create_object_pyramid1(asset_id: usize) -> Object3D {
    Object3D {
        id: 5002,
        name: Some(String::from("pyramid-1")),

        transform: OrientationBuilder::new()
            .with_transform(Matrix4x4::from(
                Vertex3D::create_x_unit(),
                Vertex3D::create_y_unit(),
                Vertex3D::create_z_unit(),
                Vertex3D::new(0.0, -1.0, -5.0),
            ))
            .build(),

        asset_id,

        visible: true,
        clipping: false,
    }
}
