use crate::graphics::storage::asset::m3d::Mesh3D;
use crate::graphics::storage::asset::material::Material;

pub mod mv3d;
pub mod m3d;
pub mod material;

// todo: this needs a better name than "asset"; it's really a model.  maybe rename after we remove the existing primitive structures

pub struct Asset3D {
    pub meshes: Vec<Mesh3D>,
    pub materials: Vec<Material>,
    pub visible: bool,
}
