use crate::geometry::primitive::v3d::Vertex3D;

// todo: documentation
impl Vertex3D {

    pub fn normalize(&mut self) {
        let magnitude_squared = self.x * self.x + self.y * self.y + self.z * self.z;
        if magnitude_squared > 0.0 {
            let one_div_mag = 1.0 / magnitude_squared.sqrt();
            self.x = self.x * one_div_mag;
            self.y = self.y * one_div_mag;
            self.z = self.z * one_div_mag;
        }
    }
}

// todo: unit tests
