use num_traits::Float;
use std::ops::{Add, Sub};

pub struct Point2D<F: Float + Sub<F> + Add<F>> {
    pub x: F,
    pub y: F,
}

impl<F: Float> Point2D<F> {
    pub fn new(x: F, y: F) -> Point2D<F> {
        Point2D { x, y }
    }

    pub fn origin() -> Point2D<F> {
        Point2D { x: F::zero(), y: F::zero() }
    }

    pub fn distance(&self, other: &Point2D<F>) -> F {
        ((self.x-other.x).powi(2) + (self.y-other.y).powi(2)).sqrt()
    }
}
