use crate::graphics::camera::Camera;
use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;
use num_traits::Float;

impl<F: Float> Default for Camera<F> {
    fn default() -> Camera<F> {
        Camera {
            orientation: Matrix4x4 {
                c4r1: F::zero(),
                c4r2: F::zero(),
                c4r3: F::from(1.5).unwrap(),
                ..Default::default()
            },

            pitch: F::zero(),
            yaw: F::zero(),

            width: F::from(800.0).unwrap(),
            height: F::from(600.0).unwrap(),

            near: F::from(0.01).unwrap(),
            far: F::from(500.0).unwrap(),
        }
    }
}
