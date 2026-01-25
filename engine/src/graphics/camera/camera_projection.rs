use num_traits::Float;
use crate::graphics::camera::Camera;

impl<F: Float> Camera<F> {
    pub fn aspect(&self) -> f64 {
        (F::from(self.width).unwrap() / F::from(self.height).unwrap()).to_f64().unwrap()
    }
}
