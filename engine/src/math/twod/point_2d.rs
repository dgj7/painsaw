pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Point2D {
        Point2D { x, y }
    }

    pub fn origin() -> Point2D {
        Point2D { x: 0.0, y: 0.0 }
    }
}
