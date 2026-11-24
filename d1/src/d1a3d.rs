//!
//! load assets.
//!

use engine::geometry::primitive::v3d::Vertex3D;
use engine::graphics::color::Color;
use engine::support::asset::{Asset3D, Asset3DLoader, Material, Mesh3D, MeshVertex3D};
use engine::support::asset::wobj::WObj;

const PYRAMID_OBJ: &str = include_str!("../../engine/tests/pyramid.obj");

pub(super) fn create_3d_pyramid_asset() -> Asset3D {
    let mut asset = WObj::load_from_bytes(PYRAMID_OBJ.as_bytes())
        .expect("pyramid.obj should load successfully");

    asset.id = 9002;
    asset.name = String::from("yellow pyramid");

    asset.materials = vec!(
        Material {
            name: Some(String::from("test-yellow")),
            diffuse_texture: None,
            diffuse_color: Color::YELLOW,

            normal_texture: None,
            specular_texture: None,
            specular_color: Color::WHITE,
            shininess: 0.0,
        },
    );

    for mesh in &mut asset.meshes {
        mesh.material_index = Some(0);
    }

    asset
}

pub(super) fn create_3d_triangle_asset() -> Asset3D {
    Asset3D {
        id: 9000,
        name: String::from("red triangle"),
        meshes: vec![
            Mesh3D {
                vertices: vec![
                    MeshVertex3D {
                        position: Vertex3D::new(-1.0, -1.0, -5.0),
                        normal: Some(Vertex3D::new(0.0, 0.0, 1.0)),
                        tex_coord: None,
                    },
                    MeshVertex3D {
                        position: Vertex3D::new(1.0, -1.0, -5.0),
                        normal: Some(Vertex3D::new(0.0, 0.0, 1.0)),
                        tex_coord: None,
                    },
                    MeshVertex3D {
                        position: Vertex3D::new(0.0, 1.0, -5.0),
                        normal: Some(Vertex3D::new(0.0, 0.0, 1.0)),
                        tex_coord: None,
                    },
                ],
                indices: vec![
                    0, 1, 2,
                ],
                material_index: Some(0),
            },
        ],
        materials: vec![
            Material {
                name: Some(String::from("test-red")),
                diffuse_texture: None,
                diffuse_color: Color::RED,

                normal_texture: None,
                specular_texture: None,
                specular_color: Color::WHITE,
                shininess: 0.0,
            },
        ],
    }
}

pub(super) fn create_3d_quad_asset() -> Asset3D {
    Asset3D {
        id: 9001,
        name: String::from("green quad"),
        meshes: vec![
            Mesh3D {
                vertices: vec![
                    MeshVertex3D {
                        position: Vertex3D::new(-1.0, -1.0, -5.0),
                        normal: Some(Vertex3D::new(0.0, 0.0, 1.0)),
                        tex_coord: None,
                    },
                    MeshVertex3D {
                        position: Vertex3D::new(1.0, -1.0, -5.0),
                        normal: Some(Vertex3D::new(0.0, 0.0, 1.0)),
                        tex_coord: None,
                    },
                    MeshVertex3D {
                        position: Vertex3D::new(1.0, 1.0, -5.0),
                        normal: Some(Vertex3D::new(0.0, 0.0, 1.0)),
                        tex_coord: None,
                    },
                    MeshVertex3D {
                        position: Vertex3D::new(-1.0, 1.0, -5.0),
                        normal: Some(Vertex3D::new(0.0, 0.0, 1.0)),
                        tex_coord: None,
                    },
                ],
                indices: vec![
                    0, 1, 2,
                    0, 2, 3,
                ],
                material_index: Some(0),
            },
        ],
        materials: vec![
            Material {
                name: Some(String::from("test-green")),
                diffuse_texture: None,
                diffuse_color: Color::GREEN,

                normal_texture: None,
                specular_texture: None,
                specular_color: Color::WHITE,
                shininess: 0.0,
            },
        ],
    }
}
