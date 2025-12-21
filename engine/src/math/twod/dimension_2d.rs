
#[derive(Clone,Debug)]
pub struct Dimension2D {
    pub height: f32,
    pub width: f32,
}

impl Dimension2D {
    pub fn new(height: f32, width: f32) -> Dimension2D {
        Dimension2D { height, width }
    }
}
