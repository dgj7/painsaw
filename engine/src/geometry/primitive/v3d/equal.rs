use crate::geometry::primitive::v3d::Vertex3D;

// todo: documentation
impl Vertex3D {
    pub fn is_equal(&self, other: &Vertex3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

// todo: unit tests
