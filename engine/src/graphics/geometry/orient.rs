use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;
use num_traits::Float;

pub mod euler;
pub mod matrix;
pub mod quaternion;

pub struct Orientation<F: Float> {
    pub position: Matrix4x4<F>,  // orientation; c1=right(x), c2=up(y), c3=forward(z/normal), c4=position
    pub scale: F,
}

impl<F: Float> Orientation<F> {
    pub fn new(position: Matrix4x4<F>, scale: F) -> Orientation<F> {
        Orientation {
            position,
            scale,
        }
    }


}
