use num_traits::Float;

pub struct MoveConfig<F: Float> {
    pub forward_speed: F,
    pub backward_speed: F,
    pub strafe_speed: F,
    pub up_speed: F,
}

impl<F: Float> Default for MoveConfig<F> {
    fn default() -> MoveConfig<F> {
        MoveConfig {
            forward_speed: F::from(5.0).unwrap(),
            backward_speed: F::one(),
            strafe_speed: F::one(),
            up_speed: F::one(),
        }
    }
}
