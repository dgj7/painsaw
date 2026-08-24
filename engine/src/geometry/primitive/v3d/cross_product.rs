use crate::geometry::primitive::v3d::Vertex3D;

// todo: unit test this
// todo: add documentation

impl Vertex3D {
    pub fn cross_product(&mut self, multiplier: &Vertex3D) {
        self.x = self.y * multiplier.z - self.z * multiplier.y;
        self.y = self.z * multiplier.x - self.x * multiplier.z;
        self.z = self.x * multiplier.y - self.y * multiplier.x;
    }

    pub fn new_cross_product(left: &Vertex3D, right: &Vertex3D) -> Vertex3D {
        // todo: change this to call the above method
        Vertex3D {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x,
        }
    }
}
