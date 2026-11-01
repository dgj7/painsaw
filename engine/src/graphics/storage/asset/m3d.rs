use crate::graphics::storage::asset::mv3d::MeshVertex3D;

pub struct Mesh3D {
    pub vertices: Vec<MeshVertex3D>,
    pub indices: Vec<u32>,
    pub material_index: Option<usize>,
}
