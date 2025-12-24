use num_traits::Num;
use crate::math::twod::point_2d::Point2D;

pub struct Line2D<F: Num> {
    pub x: Point2D<F>,
    pub y: Point2D<F>,
}

impl<F: Num> Line2D<F> {
    pub fn new(x: Point2D<F>, y: Point2D<F>) -> Line2D<F> {
        Line2D { x, y }
    }
}
