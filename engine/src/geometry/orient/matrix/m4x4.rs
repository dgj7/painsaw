pub mod eq;
pub mod invert;
pub mod mult;
pub mod mult_scalar;
pub mod scale;
pub mod column_major;
pub mod norm;

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

    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            r1c1: 1.0, r1c2: 0.0, r1c3: 0.0, r1c4: 0.0,
            r2c1: 0.0, r2c2: 1.0, r2c3: 0.0, r2c4: 0.0,
            r3c1: 0.0, r3c2: 0.0, r3c3: 1.0, r3c4: 0.0,
            r4c1: 0.0, r4c2: 0.0, r4c3: 0.0, r4c4: 1.0,
        }
    }
}

impl Default for Matrix4x4 {
    fn default() -> Matrix4x4 {
        Matrix4x4::from(Vertex3D::create_x_unit(), Vertex3D::create_y_unit(), Vertex3D::create_z_unit(), Vertex3D::origin(), )
    }
}
