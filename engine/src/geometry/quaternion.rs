use num_traits::Float;

///
/// 4d number for storing rotations or orientations.
///
/// consists of a scalar part (w, angle theta), and
/// three imaginary parts (vector of x, y, z).
///
/// doesn't suffer from gimbal lock.
///
pub struct Quaternion<F: Float> {
    pub w: F,
    pub x: F,
    pub y: F,
    pub z: F,
}


