use crate::geometry::vector::p2d::Point2D;
use num_traits::Float;
use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct Line2D<F: Float + Add<F> + Sub<F>> {
    pub x: Point2D<F>,
    pub y: Point2D<F>,
}

impl<F: Float + Add<F> + Sub<F>> Line2D<F> {
    pub fn new(x: Point2D<F>, y: Point2D<F>) -> Line2D<F> {
        Line2D { x, y }
    }
}
