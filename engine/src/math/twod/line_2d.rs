use crate::math::twod::point_2d::Point2D;

pub struct Line2D {
    pub x: Point2D,
    pub y: Point2D,
}

impl Line2D {
    pub fn new(x: Point2D, y: Point2D) -> Line2D {
        Line2D { x, y }
    }
}
