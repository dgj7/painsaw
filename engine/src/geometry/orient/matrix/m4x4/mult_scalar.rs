use crate::geometry::orient::matrix::m4x4::Matrix4x4;

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
