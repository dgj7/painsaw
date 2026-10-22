use crate::geometry::orient::matrix::m4x4::Matrix4x4;
use crate::geometry::orient::matrix::m4x4::scale::scale;
use crate::geometry::primitive::v3d::Vertex3D;

impl Matrix4x4 {
    pub fn column_major_x_right(&self) -> Vertex3D {
        Vertex3D {
            x: self.r1c1,
            y: self.r2c1,
            z: self.r3c1,
        }
    }

    pub fn column_major_y_up(&self) -> Vertex3D {
        Vertex3D {
            x: self.r1c2,
            y: self.r2c2,
            z: self.r3c2,
        }
    }

    pub fn column_major_z_forward(&self) -> Vertex3D {
        Vertex3D {
            x: self.r1c3,
            y: self.r2c3,
            z: self.r3c3,
        }
    }

    pub fn column_major_position(&self) -> Vertex3D {
        Vertex3D {
            x: self.r1c4,
            y: self.r2c4,
            z: self.r3c4,
        }
    }

    pub fn column_major_x_scale(&self) -> f32 {
        scale(self.column_major_x_right())
    }

    pub fn column_major_y_scale(&self) -> f32 {
        scale(self.column_major_y_up())
    }

    pub fn column_major_z_scale(&self) -> f32 {
        scale(self.column_major_z_forward())
    }

    pub fn column_major_update_right(&mut self, right: &Vertex3D) {
        self.r1c1 = right.x;
        self.r2c1 = right.y;
        self.r3c1 = right.z;
    }

    pub fn column_major_update_up(&mut self, up: &Vertex3D) {
        self.r1c2 = up.x;
        self.r2c2 = up.y;
        self.r3c2 = up.z;
    }

    pub fn column_major_update_forward(&mut self, forward: &Vertex3D) {
        self.r1c3 = forward.x;
        self.r2c3 = forward.y;
        self.r3c3 = forward.z;
    }

    pub fn column_major_update_position(&mut self, position: &Vertex3D) {
        self.r1c4 = position.x;
        self.r2c4 = position.y;
        self.r3c4 = position.z;
    }
}
