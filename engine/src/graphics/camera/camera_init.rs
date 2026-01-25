use num_traits::Float;
use crate::graphics::camera::Camera;
use crate::graphics::geometry::dim::Dimension2D;

impl<F: Float> Camera<F> {
    pub fn new(screen: &Dimension2D<f32>) -> Camera<F> {
        Camera {
            width: F::from(screen.width).unwrap(),
            height: F::from(screen.height).unwrap(),
            ..Default::default()
        }
    }
}
