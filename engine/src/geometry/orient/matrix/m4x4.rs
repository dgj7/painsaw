mod eq;
pub(crate) mod invert;
mod mult;
pub mod new;

use crate::geometry::primitive::v3d::Vertex3D;

///
/// store a 4x4 matrix, containing rotation, translation and scaling of an object or camera.
/// a matrix could be interpreted as column-major or row-major.
///
/// column-major(opengl, unity): each column represents 3 (basis) vectors and position:
/// 1) c1: x-axis (right)
/// 2) c2: y-axis (up)
/// 3) c3: z-axis (forward)
/// 4) c4: translation (position)
///
/// the _scale_ is stored as the _length_ of each basis vector:
/// 1) x-scale is equal to the length of the x/right vector
/// 2) y-scale is equal to the length of the y/up vector
/// 3) z-scale is equal to the length of the z/forward vector
///
/// visually, the column-major 4x4 matrix is organized as such:
/// right-x  up-x  forward-x  position-x
/// right-y  up-y  forward-y  position-y
/// right-z  up-z  forward-z  position-z
/// 0        0     0          1
///
/// row-major (directx/unreal): each row represents 3 (basis) vectors and position.
///
#[derive(Clone, Debug)]
pub struct Matrix4x4 {
    /* column1: x(right) */
    pub r1c1: f32,
    pub r2c1: f32,
    pub r3c1: f32,
    pub r4c1: f32,

    /* column2: y(up) */
    pub r1c2: f32,
    pub r2c2: f32,
    pub r3c2: f32,
    pub r4c2: f32,

    /* column3: z(forward) */
    pub r1c3: f32,
    pub r2c3: f32,
    pub r3c3: f32,
    pub r4c3: f32,

    /* column4: translation(position) */
    pub r1c4: f32,
    pub r2c4: f32,
    pub r3c4: f32,
    pub r4c4: f32,
}

///
/// vector retrieval functions
///
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
}

///
/// vector update functions
///
impl Matrix4x4 {
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

impl Default for Matrix4x4 {
    fn default() -> Matrix4x4 {
        Matrix4x4::from(Vertex3D::create_x_unit(), Vertex3D::create_y_unit(), Vertex3D::create_z_unit(), Vertex3D::origin(), )
    }
}

///
/// multiply matrix by scalar.
///
pub fn multiply_scalar(matrix: &Matrix4x4, scalar: f32) -> Matrix4x4 {
    Matrix4x4 {
        r1c1: matrix.r1c1 * scalar,
        r2c1: matrix.r2c1 * scalar,
        r3c1: matrix.r3c1 * scalar,
        r4c1: matrix.r4c1 * scalar,

        r1c2: matrix.r1c2 * scalar,
        r2c2: matrix.r2c2 * scalar,
        r3c2: matrix.r3c2 * scalar,
        r4c2: matrix.r4c2 * scalar,

        r1c3: matrix.r1c3 * scalar,
        r2c3: matrix.r2c3 * scalar,
        r3c3: matrix.r3c3 * scalar,
        r4c3: matrix.r4c3 * scalar,

        r1c4: matrix.r1c4 * scalar,
        r2c4: matrix.r2c4 * scalar,
        r3c4: matrix.r3c4 * scalar,
        r4c4: matrix.r4c4 * scalar,
    }
}

///
/// calculate the scale for the given axis.
///
/// presumes the axis is not normalized.
///
fn scale(axis: Vertex3D) -> f32 {
    ((axis.x * axis.x) + (axis.y * axis.y) + (axis.z * axis.z)).sqrt()
}

#[cfg(test)]
mod test_multiply {
    #[test]
    fn test_identity() {

    }
}
