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

impl Matrix4x4 {
    pub fn from(
        x_right: Vertex3D,
        y_up: Vertex3D,
        z_forward: Vertex3D,
        position: Vertex3D,
    ) -> Matrix4x4 {
        Matrix4x4 {
            r1c1: x_right.x,
            r2c1: x_right.y,
            r3c1: x_right.z,
            r4c1: 0.0,

            r1c2: y_up.x,
            r2c2: y_up.y,
            r3c2: y_up.z,
            r4c2: 0.0,

            r1c3: z_forward.x,
            r2c3: z_forward.y,
            r3c3: z_forward.z,
            r4c3: 0.0,

            r1c4: position.x,
            r2c4: position.y,
            r3c4: position.z,
            r4c4: 0.0, // todo: i think this should be 1.0
        }
    }

    pub fn from_position(position: Vertex3D) -> Matrix4x4 {
        Matrix4x4::from(Vertex3D::origin(), Vertex3D::origin(), Vertex3D::origin(), position,)
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            r1c1: 1.0, r1c2: 0.0, r1c3: 0.0, r1c4: 0.0,
            r2c1: 0.0, r2c2: 1.0, r2c3: 0.0, r2c4: 0.0,
            r3c1: 0.0, r3c2: 0.0, r3c3: 1.0, r3c4: 0.0,
            r4c1: 0.0, r4c2: 0.0, r4c3: 0.0, r4c4: 1.0,
        }
    }

    pub fn normalize(&mut self) {
        if let Some(x) = normalize(self) {
            *self = x;
        }
    }

    ///
    /// calculate cofactors for this matrix.
    ///
    pub fn cofactors(&self) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
        let c11 = self.r2c2 * (self.r3c3 * self.r4c4 - self.r3c4 * self.r4c3)
            - self.r2c3 * (self.r3c2 * self.r4c4 - self.r3c4 * self.r4c2)
            + self.r2c4 * (self.r3c2 * self.r4c3 - self.r3c3 * self.r4c2);
        let c12 = -(self.r2c1 * (self.r3c3 * self.r4c4 - self.r3c4 * self.r4c3)
            - self.r2c3 * (self.r3c1 * self.r4c4 - self.r3c4 * self.r4c1)
            + self.r2c4 * (self.r3c1 * self.r4c3 - self.r3c3 * self.r4c1));
        let c13 = self.r2c1 * (self.r3c2 * self.r4c4 - self.r3c4 * self.r4c2)
            - self.r2c2 * (self.r3c1 * self.r4c4 - self.r3c4 * self.r4c1)
            + self.r2c4 * (self.r3c1 * self.r4c2 - self.r3c2 * self.r4c1);
        let c14 = -(self.r2c1 * (self.r3c2 * self.r4c3 - self.r3c3 * self.r4c2)
            - self.r2c2 * (self.r3c1 * self.r4c3 - self.r3c3 * self.r4c1)
            + self.r2c3 * (self.r3c1 * self.r4c2 - self.r3c2 * self.r4c1));
        let c21 = -(self.r1c2 * (self.r3c3 * self.r4c4 - self.r3c4 * self.r4c3)
            - self.r1c3 * (self.r3c2 * self.r4c4 - self.r3c4 * self.r4c2)
            + self.r1c4 * (self.r3c2 * self.r4c3 - self.r3c3 * self.r4c2));
        let c22 = self.r1c1 * (self.r3c3 * self.r4c4 - self.r3c4 * self.r4c3)
            - self.r1c3 * (self.r3c1 * self.r4c4 - self.r3c4 * self.r4c1)
            + self.r1c4 * (self.r3c1 * self.r4c3 - self.r3c3 * self.r4c1);
        let c23 = -(self.r1c1 * (self.r3c2 * self.r4c4 - self.r3c4 * self.r4c2)
            - self.r1c2 * (self.r3c1 * self.r4c4 - self.r3c4 * self.r4c1)
            + self.r1c4 * (self.r3c1 * self.r4c2 - self.r3c2 * self.r4c1));
        let c24 = self.r1c1 * (self.r3c2 * self.r4c3 - self.r3c3 * self.r4c2)
            - self.r1c2 * (self.r3c1 * self.r4c3 - self.r3c3 * self.r4c1)
            + self.r1c3 * (self.r3c1 * self.r4c2 - self.r3c2 * self.r4c1);
        let c31 = self.r1c2 * (self.r2c3 * self.r4c4 - self.r2c4 * self.r4c3)
            - self.r1c3 * (self.r2c2 * self.r4c4 - self.r2c4 * self.r4c2)
            + self.r1c4 * (self.r2c2 * self.r4c3 - self.r2c3 * self.r4c2);
        let c32 = -(self.r1c1 * (self.r2c3 * self.r4c4 - self.r2c4 * self.r4c3)
            - self.r1c3 * (self.r2c1 * self.r4c4 - self.r2c4 * self.r4c1)
            + self.r1c4 * (self.r2c1 * self.r4c3 - self.r2c3 * self.r4c1));
        let c33 = self.r1c1 * (self.r2c2 * self.r4c4 - self.r2c4 * self.r4c2)
            - self.r1c2 * (self.r2c1 * self.r4c4 - self.r2c4 * self.r4c1)
            + self.r1c4 * (self.r2c1 * self.r4c2 - self.r2c2 * self.r4c1);
        let c34 = -(self.r1c1 * (self.r2c2 * self.r4c3 - self.r2c3 * self.r4c2)
            - self.r1c2 * (self.r2c1 * self.r4c3 - self.r2c3 * self.r4c1)
            + self.r1c3 * (self.r2c1 * self.r4c2 - self.r2c2 * self.r4c1));
        let c41 = -(self.r1c2 * (self.r2c3 * self.r3c4 - self.r2c4 * self.r3c3)
            - self.r1c3 * (self.r2c2 * self.r3c4 - self.r2c4 * self.r3c2)
            + self.r1c4 * (self.r2c2 * self.r3c3 - self.r2c3 * self.r3c2));
        let c42 = self.r1c1 * (self.r2c3 * self.r3c4 - self.r2c4 * self.r3c3)
            - self.r1c3 * (self.r2c1 * self.r3c4 - self.r2c4 * self.r3c1)
            + self.r1c4 * (self.r2c1 * self.r3c3 - self.r2c3 * self.r3c1);
        let c43 = -(self.r1c1 * (self.r2c2 * self.r3c4 - self.r2c4 * self.r3c2)
            - self.r1c2 * (self.r2c1 * self.r3c4 - self.r2c4 * self.r3c1)
            + self.r1c4 * (self.r2c1 * self.r3c2 - self.r2c2 * self.r3c1));
        let c44 = self.r1c1 * (self.r2c2 * self.r3c3 - self.r2c3 * self.r3c2)
            - self.r1c2 * (self.r2c1 * self.r3c3 - self.r2c3 * self.r3c1)
            + self.r1c3 * (self.r2c1 * self.r3c2 - self.r2c2 * self.r3c1);
        (c11,c12,c13,c14,c21,c22,c23,c24,c31,c32,c33,c34,c41,c42,c43,c44)
    }

    ///
    /// calculate the determinant of this matrix.
    ///
    pub fn determinant(&self) -> f32 {
        /* calculate cofactors */
        let (c11,c12,c13,c14,_,_,_,_,_,_,_,_,_,_,_,_) = self.cofactors();

        /* calculate determinant; consists of the first column and its cofactors */
        self.r1c1 * c11 + self.r1c2 * c12 + self.r1c3 * c13 + self.r1c4 * c14
    }
}

impl Default for Matrix4x4 {
    fn default() -> Matrix4x4 {
        Matrix4x4::from(Vertex3D::create_x_unit(), Vertex3D::create_y_unit(), Vertex3D::create_z_unit(), Vertex3D::origin(), )
    }
}

///
/// implementation of partial-eq for m4x4.
///
impl PartialEq<Self> for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.r1c1 == other.r1c1
            && self.r2c1 == other.r2c1
            && self.r3c1 == other.r3c1
            && self.r4c1 == other.r4c1
            && self.r1c2 == other.r1c2
            && self.r2c2 == other.r2c2
            && self.r3c2 == other.r3c2
            && self.r4c2 == other.r4c2
            && self.r1c3 == other.r1c3
            && self.r2c3 == other.r2c3
            && self.r3c3 == other.r3c3
            && self.r4c3 == other.r4c3
            && self.r1c4 == other.r1c4
            && self.r2c4 == other.r2c4
            && self.r3c4 == other.r3c4
            && self.r4c4 == other.r4c4
    }
}

///
/// invert the given matrix, as long as it's invertible (having determinant > 0).
///
fn invert(m: &Matrix4x4) -> Option<Matrix4x4> {
    /* get the cofactors */
    let (c11,c12,c13,c14,c21,c22,c23,c24,c31,c32,c33,c34,c41,c42,c43,c44) = m.cofactors();

    /* calculate the determinant */
    let determinant = m.determinant();

    /* if the deterrent is practically zero, return none */
    if determinant.abs() < f32::EPSILON {
        return None;
    }

    /* otherwise, get the inverse determinant */
    let inv_det = 1.0 / determinant;

    /* return a matrix, comprised of each cofactor multiplied by the inverse determinant */
    Some(Matrix4x4 {
        r1c1: c11 * inv_det,
        r2c1: c12 * inv_det,
        r3c1: c13 * inv_det,
        r4c1: c14 * inv_det,

        r1c2: c21 * inv_det,
        r2c2: c22 * inv_det,
        r3c2: c23 * inv_det,
        r4c2: c24 * inv_det,

        r1c3: c31 * inv_det,
        r2c3: c32 * inv_det,
        r3c3: c33 * inv_det,
        r4c3: c34 * inv_det,

        r1c4: c41 * inv_det,
        r2c4: c42 * inv_det,
        r3c4: c43 * inv_det,
        r4c4: c44 * inv_det,
    })
}

///
/// multiply two matrices.
///
pub fn multiply(left: &Matrix4x4, right: &Matrix4x4) -> Matrix4x4 {
    Matrix4x4 {
        r1c1: left.r1c1 * right.r1c1 + left.r1c2 * right.r2c1 + left.r1c3 * right.r3c1 + left.r1c4 * right.r4c1,
        r2c1: left.r2c1 * right.r1c1 + left.r2c2 * right.r2c1 + left.r2c3 * right.r3c1 + left.r2c4 * right.r4c1,
        r3c1: left.r3c1 * right.r1c1 + left.r3c2 * right.r2c1 + left.r3c3 * right.r3c1 + left.r3c4 * right.r4c1,
        r4c1: left.r4c1 * right.r1c1 + left.r4c2 * right.r2c1 + left.r4c3 * right.r3c1 + left.r4c4 * right.r4c1,

        r1c2: left.r1c1 * right.r1c2 + left.r1c2 * right.r2c2 + left.r1c3 * right.r3c2 + left.r1c4 * right.r4c2,
        r2c2: left.r2c1 * right.r1c2 + left.r2c2 * right.r2c2 + left.r2c3 * right.r3c2 + left.r2c4 * right.r4c2,
        r3c2: left.r3c1 * right.r1c2 + left.r3c2 * right.r2c2 + left.r3c3 * right.r3c2 + left.r3c4 * right.r4c2,
        r4c2: left.r4c1 * right.r1c2 + left.r4c2 * right.r2c2 + left.r4c3 * right.r3c2 + left.r4c4 * right.r4c2,

        r1c3: left.r1c1 * right.r1c3 + left.r1c2 * right.r2c3 + left.r1c3 * right.r3c3 + left.r1c4 * right.r4c3,
        r2c3: left.r2c1 * right.r1c3 + left.r2c2 * right.r2c3 + left.r2c3 * right.r3c3 + left.r2c4 * right.r4c3,
        r3c3: left.r3c1 * right.r1c3 + left.r3c2 * right.r2c3 + left.r3c3 * right.r3c3 + left.r3c4 * right.r4c3,
        r4c3: left.r4c1 * right.r1c3 + left.r4c2 * right.r2c3 + left.r4c3 * right.r3c3 + left.r4c4 * right.r4c3,

        r1c4: left.r1c1 * right.r1c4 + left.r1c2 * right.r2c4 + left.r1c3 * right.r3c4 + left.r1c4 * right.r4c4,
        r2c4: left.r2c1 * right.r1c4 + left.r2c2 * right.r2c4 + left.r2c3 * right.r3c4 + left.r2c4 * right.r4c4,
        r3c4: left.r3c1 * right.r1c4 + left.r3c2 * right.r2c4 + left.r3c3 * right.r3c4 + left.r3c4 * right.r4c4,
        r4c4: left.r4c1 * right.r1c4 + left.r4c2 * right.r2c4 + left.r4c3 * right.r3c4 + left.r4c4 * right.r4c4,
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
pub fn scale(axis: Vertex3D) -> f32 {
    ((axis.x * axis.x) + (axis.y * axis.y) + (axis.z * axis.z)).sqrt()
}

pub fn normalize(matrix: &Matrix4x4) -> Option<Matrix4x4> {
    let det = matrix.determinant();
    if det == 0.0 || det.is_nan() {
        return None;
    }

    let scale = 1.0 / det.abs().powf(0.25) * det.signum();

    Some(Matrix4x4 {
        r1c1: matrix.r1c1 * scale,
        r1c2: matrix.r1c2 * scale,
        r1c3: matrix.r1c3 * scale,
        r1c4: matrix.r1c4 * scale,
        r2c1: matrix.r2c1 * scale,
        r2c2: matrix.r2c2 * scale,
        r2c3: matrix.r2c3 * scale,
        r2c4: matrix.r2c4 * scale,
        r3c1: matrix.r3c1 * scale,
        r3c2: matrix.r3c2 * scale,
        r3c3: matrix.r3c3 * scale,
        r3c4: matrix.r3c4 * scale,
        r4c1: matrix.r4c1 * scale,
        r4c2: matrix.r4c2 * scale,
        r4c3: matrix.r4c3 * scale,
        r4c4: matrix.r4c4 * scale,
    })
}

///
/// inform the compiler that the partial-eq implementation represents a full equivalence relation.
///
impl Eq for Matrix4x4 {}

pub mod columnmajor {
    use crate::geometry::orient::matrix::m4x4::{scale, Matrix4x4};
    use crate::geometry::primitive::v3d::Vertex3D;

    pub fn x_right(transform: &Matrix4x4) -> Vertex3D {
        Vertex3D {
            x: transform.r1c1,
            y: transform.r2c1,
            z: transform.r3c1,
        }
    }

    pub fn y_up(transform: &Matrix4x4) -> Vertex3D {
        Vertex3D {
            x: transform.r1c2,
            y: transform.r2c2,
            z: transform.r3c2,
        }
    }

    pub fn z_forward(transform: &Matrix4x4) -> Vertex3D {
        Vertex3D {
            x: transform.r1c3,
            y: transform.r2c3,
            z: transform.r3c3,
        }
    }

    pub fn position(transform: &Matrix4x4) -> Vertex3D {
        Vertex3D {
            x: transform.r1c4,
            y: transform.r2c4,
            z: transform.r3c4,
        }
    }

    pub fn x_scale(transform: &Matrix4x4) -> f32 {
        scale(x_right(transform))
    }

    pub fn y_scale(transform: &Matrix4x4) -> f32 {
        scale(y_up(transform))
    }

    pub fn z_scale(transform: &Matrix4x4) -> f32 {
        scale(z_forward(transform))
    }

    pub fn update_right(transform: &mut Matrix4x4, right: &Vertex3D) {
        transform.r1c1 = right.x;
        transform.r2c1 = right.y;
        transform.r3c1 = right.z;
    }

    pub fn update_up(transform: &mut Matrix4x4, up: &Vertex3D) {
        transform.r1c2 = up.x;
        transform.r2c2 = up.y;
        transform.r3c2 = up.z;
    }

    pub fn update_forward(transform: &mut Matrix4x4, forward: &Vertex3D) {
        transform.r1c3 = forward.x;
        transform.r2c3 = forward.y;
        transform.r3c3 = forward.z;
    }

    pub fn update_position(transform: &mut Matrix4x4, position: &Vertex3D) {
        transform.r1c4 = position.x;
        transform.r2c4 = position.y;
        transform.r3c4 = position.z;
    }
}

#[cfg(test)]
mod tests {
    mod equals {
        use crate::geometry::orient::matrix::m4x4::Matrix4x4;

        #[test]
        fn test_equal_1() {
            let a = Matrix4x4::identity();
            let b = Matrix4x4::identity();

            assert_eq!(a, b);
        }

        #[test]
        fn test_not_equal_1() {
            let a = Matrix4x4::identity();
            let mut b = Matrix4x4::identity();

            b.r2c3 = 3.0;

            assert_ne!(a, b);
        }
    }

    mod invert {
        use crate::geometry::is_near;
        use crate::geometry::orient::matrix::m4x4::{invert, multiply, Matrix4x4};

        ///
        /// inverse of identity should be _very close_ to the identity.
        ///
        #[test]
        fn test_inverse_identity() {
            let matrix = Matrix4x4::identity();
            let inverse = invert(&matrix).unwrap();

            assert!(is_near(matrix.r1c1, inverse.r1c1));
            assert!(is_near(matrix.r2c1, inverse.r2c1));
            assert!(is_near(matrix.r3c1, inverse.r3c1));
            assert!(is_near(matrix.r4c1, inverse.r4c1));

            assert!(is_near(matrix.r1c2, inverse.r1c2));
            assert!(is_near(matrix.r2c2, inverse.r2c2));
            assert!(is_near(matrix.r1c2, inverse.r3c2));
            assert!(is_near(matrix.r1c2, inverse.r4c2));

            assert!(is_near(matrix.r1c3, inverse.r1c3));
            assert!(is_near(matrix.r2c3, inverse.r2c3));
            assert!(is_near(matrix.r3c3, inverse.r3c3));
            assert!(is_near(matrix.r4c3, inverse.r4c3));

            assert!(is_near(matrix.r1c4, inverse.r1c4));
            assert!(is_near(matrix.r2c4, inverse.r2c4));
            assert!(is_near(matrix.r3c4, inverse.r3c4));
            assert!(is_near(matrix.r4c4, inverse.r4c4));
        }

        ///
        /// a matrix times its own inverse equals identity matrix
        ///
        #[test]
        fn test_inverse_1() {
            let matrix = Matrix4x4 {
                r1c1: 1.0,
                r2c1: 0.0,
                r3c1: 1.0,
                r4c1: 0.0,
                r1c2: 0.0,
                r2c2: 3.0,
                r3c2: 0.0,
                r4c2: 2.0,
                r1c3: 2.0,
                r2c3: 0.0,
                r3c3: 1.0,
                r4c3: 0.0,
                r1c4: 0.0,
                r2c4: 4.0,
                r3c4: 0.0,
                r4c4: 3.0,
            };
            let inverse = invert(&matrix).expect("couldn't invert matrix");

            let result = multiply(&matrix, &inverse);
            let expected = Matrix4x4::identity();

            assert!(is_near(expected.r1c1, result.r1c1));
            assert!(is_near(expected.r2c1, result.r2c1));
            assert!(is_near(expected.r3c1, result.r3c1));
            assert!(is_near(expected.r4c1, result.r4c1));

            assert!(is_near(expected.r1c2, result.r1c2));
            assert!(is_near(expected.r2c2, result.r2c2));
            assert!(is_near(expected.r1c2, result.r3c2));
            assert!(is_near(expected.r1c2, result.r4c2));

            assert!(is_near(expected.r1c3, result.r1c3));
            assert!(is_near(expected.r2c3, result.r2c3));
            assert!(is_near(expected.r3c3, result.r3c3));
            assert!(is_near(expected.r4c3, result.r4c3));

            assert!(is_near(expected.r1c4, result.r1c4));
            assert!(is_near(expected.r2c4, result.r2c4));
            assert!(is_near(expected.r3c4, result.r3c4));
            assert!(is_near(expected.r4c4, result.r4c4));
        }

        ///
        /// test that a singular matrix won't be inverted
        ///
        #[test]
        fn test_inverse_singular_1() {
            let matrix = Matrix4x4 {
                r1c1: 1.0,
                r2c1: 5.0,
                r3c1: 9.0,
                r4c1: 13.0,
                r1c2: 2.0,
                r2c2: 6.0,
                r3c2: 10.0,
                r4c2: 14.0,
                r1c3: 3.0,
                r2c3: 7.0,
                r3c3: 11.0,
                r4c3: 15.0,
                r1c4: 4.0,
                r2c4: 8.0,
                r3c4: 12.0,
                r4c4: 16.0,
            };
            assert!(invert(&matrix).is_none());
        }
    }

    mod multiply {
        use crate::geometry::orient::matrix::m4x4::{multiply, Matrix4x4};

        #[test]
        fn test_mult_by_identity() {
            let identity = Matrix4x4::identity();
            let input = Matrix4x4 {
                r1c1: 1.0,
                r1c2: 2.0,
                r1c3: 3.0,
                r1c4: 4.0,
                r2c1: 5.0,
                r2c2: 6.0,
                r2c3: 7.0,
                r2c4: 8.0,
                r3c1: 9.0,
                r3c2: 10.0,
                r3c3: 11.0,
                r3c4: 12.0,
                r4c1: 13.0,
                r4c2: 14.0,
                r4c3: 15.0,
                r4c4: 16.0,
            };

            let result = multiply(&input, &identity);

            /* expected that A*identity=A */
            assert_eq!(input, result);
        }

        #[test]
        fn test_scenario1() {
            let m1 = Matrix4x4 {
                r1c1: 1.0,
                r1c2: 2.0,
                r1c3: 3.0,
                r1c4: 4.0,
                r2c1: 5.0,
                r2c2: 6.0,
                r2c3: 7.0,
                r2c4: 8.0,
                r3c1: 9.0,
                r3c2: 10.0,
                r3c3: 11.0,
                r3c4: 12.0,
                r4c1: 13.0,
                r4c2: 14.0,
                r4c3: 15.0,
                r4c4: 16.0,
            };
            let m2 = Matrix4x4 {
                r1c1: 2.0,
                r1c2: 0.0,
                r1c3: 1.0,
                r1c4: 3.0,
                r2c1: 1.0,
                r2c2: 4.0,
                r2c3: 0.0,
                r2c4: 1.0,
                r3c1: 0.0,
                r3c2: 1.0,
                r3c3: 2.0,
                r3c4: 0.0,
                r4c1: 3.0,
                r4c2: 0.0,
                r4c3: 1.0,
                r4c4: 2.0,
            };
            let expected = Matrix4x4 {
                r1c1: 16.0,
                r1c2: 11.0,
                r1c3: 11.0,
                r1c4: 13.0,
                r2c1: 40.0,
                r2c2: 31.0,
                r2c3: 27.0,
                r2c4: 37.0,
                r3c1: 64.0,
                r3c2: 51.0,
                r3c3: 43.0,
                r3c4: 61.0,
                r4c1: 88.0,
                r4c2: 71.0, r4c3: 59.0, r4c4: 85.0,
            };

            let result = multiply(&m1, &m2);

            assert_eq!(expected, result);

        }
    }

    mod normalize {
        use crate::geometry::orient::matrix::m4x4::{normalize, Matrix4x4};

        #[test]
        fn test_identity() {
            let m = Matrix4x4::identity();
            let n = normalize(&m).unwrap();

            assert_eq!(m, n);
            assert_eq!(1.0, n.determinant());
        }

        #[test]
        fn test_scaled() {
            let mut m = Matrix4x4::identity();
            m.r1c1 = 3.0;
            m.r2c2 = 3.0;
            m.r3c3 = 3.0;
            m.r4c4 = 3.0;

            assert_eq!(81.0, m.determinant());
        }

        ///
        /// a row of zeros results in determinant of 0.
        ///
        #[test]
        fn test_singular() {
            let mut m = Matrix4x4::identity();
            m.r1c1 = 0.0;
            m.r1c2 = 0.0;
            m.r1c3 = 0.0;
            m.r1c4 = 0.0;

            assert_eq!(0.0, m.determinant());
            assert!(normalize(&m).is_none());
        }
    }

    mod determinant {
        use crate::geometry::orient::matrix::m4x4::Matrix4x4;

        #[test]
        fn test_identity() {
            let m = Matrix4x4::identity();
            assert_eq!(1.0, m.determinant());
        }

        #[test]
        fn test_diagonal_scaled() {
            let mut m = Matrix4x4::identity();
            m.r1c1 = 2.0;
            m.r2c2 = 3.0;
            m.r3c3 = 4.0;
            m.r4c4 = 0.5;

            assert_eq!(12.0, m.determinant());
        }

        ///
        /// a singular matrix has two identical rows; determinant is zero
        ///
        #[test]
        fn test_singular() {
            let mut m = Matrix4x4::identity();
            m.r2c1 = m.r1c1;
            m.r2c2 = m.r1c2;
            m.r2c3 = m.r1c3;
            m.r2c4 = m.r1c4;

            assert_eq!(0.0, m.determinant());
        }

        #[test]
        fn test_scenario1() {
            let m = Matrix4x4 {
                r1c1: 1.0,
                r1c2: 0.0,
                r1c3: 2.0,
                r1c4: -1.0,
                r2c1: 3.0,
                r2c2: 0.0,
                r2c3: 0.5,
                r2c4: 2.0,
                r3c1: 2.0,
                r3c2: 1.0,
                r3c3: 4.0,
                r3c4: 0.0,
                r4c1: 0.0, r4c2: 0.0, r4c3: 3.0, r4c4: 1.0,
            };

            assert_eq!(20.5, m.determinant());
        }
    }
}
