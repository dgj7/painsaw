use crate::geometry::orient::matrix::m3x3::Matrix3x3;

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
