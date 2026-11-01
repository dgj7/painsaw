//!
//! storage for reusable assets/models.
//!
//! assets themselves are not rendered; they are referenced some number of times by objects.
//!
use crate::graphics::storage::asset::m3d::Mesh3D;
use crate::graphics::storage::asset::material::Material;

pub mod mv3d;
pub mod m3d;
pub mod material;

pub struct Asset3D {
    /* identify the asset so that it's reusable */
    pub id: usize,
    pub name: String,
    
    /* rendering data */
    pub meshes: Vec<Mesh3D>,
    pub materials: Vec<Material>,
}
