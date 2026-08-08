
#[derive(Clone, Debug)]
pub struct MouseDelta {
    pub dx: f32,
    pub dy: f32,
}

impl MouseDelta {
    pub fn new(dx: f32, dy: f32) -> MouseDelta {
        MouseDelta { dx, dy }
    }
}
