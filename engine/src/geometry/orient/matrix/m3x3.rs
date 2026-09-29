pub mod rot;

///
/// store a 3x3 matrix, containing rotation and scaling of an object or camera.
/// a matrix could be interpreted as column-major or row-major.
///
/// column-major(opengl, unity): each column represents 3 (basis) vectors:
/// 1) c1: x-axis (right)
/// 2) c2: y-axis (up)
/// 3) c3: z-axis (forward)
/// as well as the scale, which is stored in the _length_ of each vector.
/// 1) x-scale is equal to the length of the x/right vector
/// 2) y-scale is equal to the length of the y/up vector
/// 3) z-scale is equal to the length of the z/forward vector
///
/// visually, the column-major 3x3 matrix is organized as such:
/// right-x  up-x  forward-x
/// right-y  up-y  forward-y
/// right-z  up-z  forward-z
///
/// row-major (directx/unreal): each row represents 3 (basis) vectors.
///
#[derive(Debug, PartialEq)]
pub struct Matrix3x3 {
    /* row 1 */
    pub r1c1: f32,
    pub r1c2: f32,
    pub r1c3: f32,

    /* row 2 */
    pub r2c1: f32,
    pub r2c2: f32,
    pub r2c3: f32,

    /* row 3 */
    pub r3c1: f32,
    pub r3c2: f32,
    pub r3c3: f32,
}

type RotationMatrix = Matrix3x3;

impl Matrix3x3 {
    pub fn identity() -> Matrix3x3 {
        Matrix3x3 {
            r1c1: 1.0, r1c2: 0.0, r1c3: 0.0,
            r2c1: 0.0, r2c2: 1.0, r2c3: 0.0,
            r3c1: 0.0, r3c2: 0.0, r3c3: 1.0,
        }
    }

    pub fn multiply(&self, other: &Matrix3x3) -> Matrix3x3 {
        multiply(self, other)
    }
}

///
/// multiply two matrices.
///
pub fn multiply(left: &Matrix3x3, right: &Matrix3x3) -> Matrix3x3 {
    Matrix3x3 {
        r1c1: left.r1c1 * right.r1c1 + left.r1c2 * right.r2c1 + left.r1c3 * right.r3c1,
        r2c1: left.r2c1 * right.r1c1 + left.r2c2 * right.r2c1 + left.r2c3 * right.r3c1,
        r3c1: left.r3c1 * right.r1c1 + left.r3c2 * right.r2c1 + left.r3c3 * right.r3c1,

        r1c2: left.r1c1 * right.r1c2 + left.r1c2 * right.r2c2 + left.r1c3 * right.r3c2,
        r2c2: left.r2c1 * right.r1c2 + left.r2c2 * right.r2c2 + left.r2c3 * right.r3c2,
        r3c2: left.r3c1 * right.r1c2 + left.r3c2 * right.r2c2 + left.r3c3 * right.r3c2,

        r1c3: left.r1c1 * right.r1c3 + left.r1c2 * right.r2c3 + left.r1c3 * right.r3c3,
        r2c3: left.r2c1 * right.r1c3 + left.r2c2 * right.r2c3 + left.r2c3 * right.r3c3,
        r3c3: left.r3c1 * right.r1c3 + left.r3c2 * right.r2c3 + left.r3c3 * right.r3c3,
    }
}

///
/// multiply matrix by scalar.
///
pub fn multiply_scalar(matrix: &Matrix3x3, scalar: f32) -> Matrix3x3 {
    Matrix3x3 {
        r1c1: matrix.r1c1 * scalar,
        r2c1: matrix.r2c1 * scalar,
        r3c1: matrix.r3c1 * scalar,

        r1c2: matrix.r1c2 * scalar,
        r2c2: matrix.r2c2 * scalar,
        r3c2: matrix.r3c2 * scalar,

        r1c3: matrix.r1c3 * scalar,
        r2c3: matrix.r2c3 * scalar,
        r3c3: matrix.r3c3 * scalar,
    }
}

#[cfg(test)]
mod test_multiply {
    use crate::geometry::orient::matrix::m3x3::{multiply, Matrix3x3};

    #[test]
    fn test_identity() {
        let m1 = Matrix3x3 {
            r1c1: 1.0, r1c2: 2.0, r1c3: 3.0,
            r2c1: 4.0, r2c2: 5.0, r2c3: 6.0,
            r3c1: 7.0, r3c2: 8.0, r3c3: 9.0,
        };
        let id = Matrix3x3::identity();

        let result = multiply(&m1, &id);

        assert_eq!(m1, result);
    }

    #[test]
    fn test_scenario1() {
        let m1 = Matrix3x3 {
            r1c1: 1.0, r1c2: 2.0, r1c3: 3.0,
            r2c1: 4.0, r2c2: 5.0, r2c3: 6.0,
            r3c1: 7.0, r3c2: 8.0, r3c3: 9.0,
        };
        let m2 = Matrix3x3 {
            r1c1: 9.0, r1c2: 8.0, r1c3: 7.0,
            r2c1: 6.0, r2c2: 5.0, r2c3: 4.0,
            r3c1: 3.0, r3c2: 2.0, r3c3: 1.0,
        };
        let expected = Matrix3x3 {
            r1c1: 30.0, r1c2: 24.0, r1c3: 18.0,
            r2c1: 84.0, r2c2: 69.0, r2c3: 54.0,
            r3c1: 138.0, r3c2: 114.0, r3c3: 90.0,
        };

        let result = multiply(&m1, &m2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_scenario2() {
        let m1 = Matrix3x3 {
            r1c1: 1.0, r1c2: 2.0, r1c3: 3.0,
            r2c1: 4.0, r2c2: 5.0, r2c3: 6.0,
            r3c1: 7.0, r3c2: 8.0, r3c3: 9.0,
        };
        let m2 = Matrix3x3 {
            r1c1: 10.0, r1c2: 11.0, r1c3: 12.0,
            r2c1: 13.0, r2c2: 14.0, r2c3: 15.0,
            r3c1: 16.0, r3c2: 17.0, r3c3: 18.0,
        };
        let expected = Matrix3x3 {
            r1c1: 84.0, r1c2: 90.0, r1c3: 96.0,
            r2c1: 201.0, r2c2: 216.0, r2c3: 231.0,
            r3c1: 318.0, r3c2: 342.0, r3c3: 366.0,
        };

        let result = multiply(&m1, &m2);

        assert_eq!(expected, result);
    }
}
