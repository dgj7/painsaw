pub mod column_major;
mod mult;
mod mult_scalar;

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
}
