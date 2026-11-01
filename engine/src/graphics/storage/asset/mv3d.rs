use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::v3d::Vertex3D;

pub struct MeshVertex3D {
    pub position: Vertex3D,
    pub normal: Option<Vertex3D>,
    pub tex_coord: Option<Vertex2D>,
}
