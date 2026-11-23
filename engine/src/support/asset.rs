//!
//! storage for reusable assets/models.
//!
//! assets themselves are not rendered; they are referenced some number of times by objects.
//!

mod wobj;

use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek};
use std::path::Path;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::v3d::Vertex3D;
use crate::graphics::color::Color;
use crate::graphics::texture::Texture;

pub struct Asset3D {
    /* identify the asset so that it's reusable */
    pub id: usize,
    pub name: String,

    /* rendering data */
    pub meshes: Vec<Mesh3D>,
    pub materials: Vec<Material>,
}

pub struct Mesh3D {
    pub vertices: Vec<MeshVertex3D>,
    pub indices: Vec<u32>,
    pub material_index: Option<usize>,
}

pub struct MeshVertex3D {
    pub position: Vertex3D,
    pub normal: Option<Vertex3D>,
    pub tex_coord: Option<Vertex2D>,
}

pub struct Material {
    pub name: Option<String>,
    pub diffuse_texture: Option<Texture>,
    pub diffuse_color: Color,

    // todo: for later
    pub normal_texture: Option<Texture>,
    pub specular_texture: Option<Texture>,
    pub specular_color: Color,
    pub shininess: f32,
}

pub trait Asset3DLoader {
    fn load_from_buf_read<R: BufRead + Seek>(reader: R) -> std::io::Result<Asset3D>;

    fn load_from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Asset3D> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::load_from_buf_read(&mut reader)
    }

    fn load_from_bytes(bytes: &[u8]) -> std::io::Result<Asset3D> {
        let mut cursor = Cursor::new(bytes);
        Self::load_from_buf_read(&mut cursor)
    }
}
