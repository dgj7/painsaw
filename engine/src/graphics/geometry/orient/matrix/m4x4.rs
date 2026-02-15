use crate::graphics::geometry::primitive::v3d::Vertex3D;

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
#[derive(Clone)]
pub struct Matrix4x4 {
    /* column1: x(right) */
    pub c1r1: f32,
    pub c1r2: f32,
    pub c1r3: f32,
    pub c1r4: f32,

    /* column2: y(up) */
    pub c2r1: f32,
    pub c2r2: f32,
    pub c2r3: f32,
    pub c2r4: f32,

    /* column3: z(forward) */
    pub c3r1: f32,
    pub c3r2: f32,
    pub c3r3: f32,
    pub c3r4: f32,

    /* column4: translation(position) */
    pub c4r1: f32,
    pub c4r2: f32,
    pub c4r3: f32,
    pub c4r4: f32,
}

impl Matrix4x4 {
    pub fn from(x_right: Vertex3D, y_up: Vertex3D, z_forward: Vertex3D, position: Vertex3D) -> Matrix4x4 {
        Matrix4x4 {
            c1r1: x_right.x,
            c1r2: x_right.y,
            c1r3: x_right.z,
            c1r4: 0.0,

            c2r1: y_up.x,
            c2r2: y_up.y,
            c2r3: y_up.z,
            c2r4: 0.0,


            c3r1: z_forward.x,
            c3r2: z_forward.y,
            c3r3: z_forward.z,
            c3r4: 0.0,

            c4r1: position.x,
            c4r2: position.y,
            c4r3: position.z,
            c4r4: 0.0,// todo: i think this should be 1.0
        }
    }
}

impl Matrix4x4 {
    pub fn column_major_x_right(&self) -> Vertex3D {
        Vertex3D {
            x: self.c1r1,
            y: self.c1r2,
            z: self.c1r3,
        }
    }

    pub fn column_major_y_up(&self) -> Vertex3D {
        Vertex3D {
            x: self.c2r1,
            y: self.c2r2,
            z: self.c2r3,
        }
    }

    pub fn column_major_z_forward(&self) -> Vertex3D {
        Vertex3D {
            x: self.c3r1,
            y: self.c3r2,
            z: self.c3r3,
        }
    }

    pub fn column_major_position(&self) -> Vertex3D {
        Vertex3D {
            x: self.c4r1,
            y: self.c4r2,
            z: self.c4r3,
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

    pub fn column_major_update_position(&mut self, position: &Vertex3D) {
        self.c4r1 = position.x;
        self.c4r2 = position.y;
        self.c4r3 = position.z;
    }
}

impl Default for Matrix4x4 {
    fn default() -> Matrix4x4 {
        Matrix4x4::from(Vertex3D::create_x_unit(), Vertex3D::create_y_unit(), Vertex3D::create_z_unit(), Vertex3D::origin())
    }
}

///
/// multiply two matrices.
///
pub fn multiply(left: &Matrix4x4, right: &Matrix4x4) -> Matrix4x4 {
    Matrix4x4 {
        c1r1: left.c1r1* right.c1r1 + left.c2r1* right.c1r2 + left.c3r1* right.c1r3 + left.c4r1* right.c1r4,
        c1r2: left.c1r2* right.c1r1 + left.c2r2* right.c1r2 + left.c3r2* right.c1r3 + left.c4r2* right.c1r4,
        c1r3: left.c1r3* right.c1r1 + left.c2r3* right.c1r2 + left.c3r3* right.c1r3 + left.c4r3* right.c1r4,
        c1r4: left.c1r4* right.c1r1 + left.c2r4* right.c1r2 + left.c3r4* right.c1r3 + left.c4r4* right.c1r4,

        c2r1: left.c1r1* right.c2r1 + left.c2r1* right.c2r2 + left.c3r1* right.c2r3 + left.c4r1* right.c2r4,
        c2r2: left.c1r2* right.c2r1 + left.c2r2* right.c2r2 + left.c3r2* right.c2r3 + left.c4r2* right.c2r4,
        c2r3: left.c1r3* right.c2r1 + left.c2r3* right.c2r2 + left.c3r3* right.c2r3 + left.c4r3* right.c2r4,
        c2r4: left.c1r4* right.c2r1 + left.c2r4* right.c2r2 + left.c3r4* right.c2r3 + left.c4r4* right.c2r4,

        c3r1: left.c1r1* right.c3r1 + left.c2r1* right.c3r2 + left.c3r1* right.c3r3 + left.c4r1* right.c3r4,
        c3r2: left.c1r2* right.c3r1 + left.c2r2* right.c3r2 + left.c3r2* right.c3r3 + left.c4r2* right.c3r4,
        c3r3: left.c1r3* right.c3r1 + left.c2r3* right.c3r2 + left.c3r3* right.c3r3 + left.c4r3* right.c3r4,
        c3r4: left.c1r4* right.c3r1 + left.c2r4* right.c3r2 + left.c3r4* right.c3r3 + left.c4r4* right.c3r4,

        c4r1: left.c1r1* right.c4r1 + left.c2r1* right.c4r2 + left.c3r1* right.c4r3 + left.c4r1* right.c4r4,
        c4r2: left.c1r2* right.c4r1 + left.c2r2* right.c4r2 + left.c3r2* right.c4r3 + left.c4r2* right.c4r4,
        c4r3: left.c1r3* right.c4r1 + left.c2r3* right.c4r2 + left.c3r3* right.c4r3 + left.c4r3* right.c4r4,
        c4r4: left.c1r4* right.c4r1 + left.c2r4* right.c4r2 + left.c3r4* right.c4r3 + left.c4r4* right.c4r4,
    }
}

///
/// multiply matrix by scalar.
///
pub fn multiply_scalar(matrix: &Matrix4x4, scalar: f32) -> Matrix4x4 {
    Matrix4x4 {
        c1r1: matrix.c1r1 * scalar,
        c1r2: matrix.c1r2 * scalar,
        c1r3: matrix.c1r3 * scalar,
        c1r4: matrix.c1r4 * scalar,

        c2r1: matrix.c2r1 * scalar,
        c2r2: matrix.c2r2 * scalar,
        c2r3: matrix.c2r3 * scalar,
        c2r4: matrix.c2r4 * scalar,

        c3r1: matrix.c3r1 * scalar,
        c3r2: matrix.c3r2 * scalar,
        c3r3: matrix.c3r3 * scalar,
        c3r4: matrix.c3r4 * scalar,

        c4r1: matrix.c4r1 * scalar,
        c4r2: matrix.c4r2 * scalar,
        c4r3: matrix.c4r3 * scalar,
        c4r4: matrix.c4r4 * scalar,
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
