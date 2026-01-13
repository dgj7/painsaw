use num_traits::Float;

///
/// three rotations, which describe the orientation of an object in 3d space.
///
/// can succumb to gimbal lock.
///
pub struct EulerAngles<F: Float> {
    pub heading: F,
    pub pitch: F,
    pub bank: F,
}

impl<F: Float> EulerAngles<F> {
    pub fn identity() -> EulerAngles<F> {
        EulerAngles { heading: F::zero(), pitch: F::zero(), bank: F::zero() }
    }
}
