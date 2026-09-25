#[derive(Clone, Debug)]
pub struct MoveConfig {
    pub forward_speed: f32,
    pub backward_speed: f32,
    pub strafe_speed: f32,
    pub up_speed: f32,
}

impl Default for MoveConfig {
    fn default() -> MoveConfig {
        MoveConfig {
            forward_speed: 5.0,
            backward_speed: 1.0,
            strafe_speed: 1.0,
            up_speed: 1.0,
        }
    }
}
