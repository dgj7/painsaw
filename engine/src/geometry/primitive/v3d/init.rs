use crate::geometry::primitive::v3d::Vertex3D;

///
/// methods related to initialization.
///
impl Vertex3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex3D {
        Vertex3D { x, y, z }
    }

    pub fn origin() -> Vertex3D {
        Vertex3D::new(0.0, 0.0, 0.0)
    }

    pub fn create_x_unit() -> Vertex3D {
        Vertex3D::new(1.0, 0.0, 0.0)
    }

    pub fn create_y_unit() -> Vertex3D {
        Vertex3D::new(0.0, 1.0, 0.0)
    }

    pub fn create_z_unit() -> Vertex3D {
        Vertex3D::new(0.0, 0.0, 1.0)
    }
}
