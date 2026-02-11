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
