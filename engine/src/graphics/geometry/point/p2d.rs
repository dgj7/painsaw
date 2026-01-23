use num_traits::Float;
use std::ops::{Add, Sub};

#[derive(Clone)]
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

#[cfg(test)]
mod point2d_distance_tests {
    use crate::graphics::geometry::point::p2d::Point2D;

    #[test]
    fn test_positive1() {
        let left = Point2D::new(3.0, 1.0);
        let right = Point2D::new(6.0, 5.0);

        let dist = left.distance(&right);

        assert_eq!(5.0, dist);
    }

    #[test]
    fn test_mixed1() {
        let left = Point2D::new(-1.0, 3.0);
        let right = Point2D::new(2.0, -2.0);

        let dist = left.distance(&right);

        assert_eq!(5.830951894845301, dist);
    }

    #[test]
    fn test_mixed2() {
        let left = Point2D::new(2.0, -6.0);
        let right = Point2D::new(7.0, 3.0);

        let dist = left.distance(&right);

        assert_eq!(10.295630140987, dist);
    }
}
