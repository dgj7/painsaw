pub mod camera_default;
pub mod camera_init;
pub mod camera_update;
pub mod camera_projection;
pub mod camera_move;

use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;
use num_traits::Float;

pub struct Camera<F: Float> {
    pub orientation: Matrix4x4<F>,  // orientation; c1=right(x), c2=up(y), c3=forward(z/normal), c4=position

    pub pitch: F,                   // x-axis rotation
    pub yaw: F,                     // y-axis rotation

    pub width: F,                   // window width
    pub height: F,                  // window height

    pub near: F,                    // 3d near clipping plane
    pub far: F,                     // 3d far clipping plane
}
