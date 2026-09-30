use crate::geometry::orient::matrix::m4x4::Matrix4x4;

impl Matrix4x4 {
    // todo: update this function to stop using column_major functions specifically
    pub fn normalize(&mut self) {
        let mut forward = self.column_major_z_forward();
        forward.normalize();
        self.column_major_update_forward(&forward);

        let mut right = self.column_major_x_right();
        right.normalize();
        self.column_major_update_right(&right);

        let mut up = self.column_major_y_up();
        up.normalize();
        self.column_major_update_up(&up);
    }
}
