use num_traits::Float;
use std::ops::{Add, Sub};

pub struct Point3D<F: Float + Add<F> + Sub<F>> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float + Add<F> + Sub<F>> Point3D<F> {
    pub fn new(x: F, y: F, z: F) -> Point3D<F> {
        Point3D { x, y, z }
    }

    pub fn origin() -> Point3D<F> {
        Point3D::new(F::zero(), F::zero(), F::zero())
    }

    pub fn distance(&self, other: &Point3D<F>) -> F {
        ((self.x-other.x).powi(2) + (self.y-other.y).powi(2) + (self.z-other.z).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod point3d_distance_tests {
    use crate::math::threed::point_3d::Point3D;

    #[test]
    fn test_positive1() {
        let left = Point3D::new(2.0, 3.0, 4.0);
        let right = Point3D::new(5.0,7.0, 9.0);

        let dist = left.distance(&right);

        assert_eq!(7.0710678118654755, dist);
    }

    #[test]
    fn test_mixed1() {
        let left = Point3D::new(-3.0, 4.0, -2.0);
        let right = Point3D::new(1.0, -1.0, 3.0);

        let dist = left.distance(&right);

        assert_eq!(8.12403840463596, dist);
    }
}
