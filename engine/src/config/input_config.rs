pub mod kc;
pub mod mc;

#[derive(Clone, Debug)]
pub struct InputConfig {
    pub mouse_sensitivity: f32,
}

impl InputConfig {
    pub fn new(mouse_sensitivity: f32) -> InputConfig {
        InputConfig { mouse_sensitivity }
    }
}

///
/// default input config.
///
impl Default for InputConfig {
    fn default() -> InputConfig {
        InputConfig {
            mouse_sensitivity: 1.0,
        }
    }
}
