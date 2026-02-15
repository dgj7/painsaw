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
pub struct Matrix3x3 {
    /* column 1 */
    pub c1r1: f32,
    pub c1r2: f32,
    pub c1r3: f32,

    /* column 2 */
    pub c2r1: f32,
    pub c2r2: f32,
    pub c2r3: f32,

    /* column 3 */
    pub c3r1: f32,
    pub c3r2: f32,
    pub c3r3: f32,
}

#[allow(dead_code)] // todo: remove this
type RotationMatrix = Matrix3x3;

///
/// multiply two matrices.
///
pub fn multiply(left: &Matrix3x3, right: &Matrix3x3) -> Matrix3x3 {
    Matrix3x3 {
        c1r1: left.c1r1* right.c1r1 + left.c2r1* right.c1r2 + left.c3r1* right.c1r3,
        c1r2: left.c1r2* right.c1r1 + left.c2r2* right.c1r2 + left.c3r2* right.c1r3,
        c1r3: left.c1r3* right.c1r1 + left.c2r3* right.c1r2 + left.c3r3* right.c1r3,

        c2r1: left.c1r1* right.c2r1 + left.c2r1* right.c2r2 + left.c3r1* right.c2r3,
        c2r2: left.c1r2* right.c2r1 + left.c2r2* right.c2r2 + left.c3r2* right.c2r3,
        c2r3: left.c1r3* right.c2r1 + left.c2r3* right.c2r2 + left.c3r3* right.c2r3,

        c3r1: left.c1r1* right.c3r1 + left.c2r1* right.c3r2 + left.c3r1* right.c3r3,
        c3r2: left.c1r2* right.c3r1 + left.c2r2* right.c3r2 + left.c3r2* right.c3r3,
        c3r3: left.c1r3* right.c3r1 + left.c2r3* right.c3r2 + left.c3r3* right.c3r3,
    }
}

///
/// multiply matrix by scalar.
///
pub fn multiply_scalar(matrix: &Matrix3x3, scalar: f32) -> Matrix3x3 {
    Matrix3x3 {
        c1r1: matrix.c1r1 * scalar,
        c1r2: matrix.c1r2 * scalar,
        c1r3: matrix.c1r3 * scalar,

        c2r1: matrix.c2r1 * scalar,
        c2r2: matrix.c2r2 * scalar,
        c2r3: matrix.c2r3 * scalar,

        c3r1: matrix.c3r1 * scalar,
        c3r2: matrix.c3r2 * scalar,
        c3r3: matrix.c3r3 * scalar,
    }
}
