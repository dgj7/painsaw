use num_traits::Float;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone)]
pub struct Point3D<F: Float + Add<F> + Sub<F> + Mul<F> + Div<F>> {
    pub x: F,
    pub y: F,
    pub z: F,
}

#[allow(dead_code)] // todo: remove this
type Vector3D<F> = Point3D<F>;

impl<F: Float + Add<F> + Sub<F> + Mul<F> + Div<F>> Point3D<F> {

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn is_equal(&self, other: &Point3D<F>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    pub fn normalize(&mut self) {
        let magnitude_squared = self.x * self.x + self.y * self.y + self.z * self.z;
        if magnitude_squared > F::zero() {
            let one_div_mag = F::one() / magnitude_squared.sqrt();
            self.x = self.x * one_div_mag;
            self.y = self.y * one_div_mag;
            self.z = self.z * one_div_mag;
        }
    }

    pub fn distance_to(&self, other: &Point3D<F>) -> F {
        distance(self, other)
    }

    pub fn add(&mut self, addend: &Point3D<F>) {
        self.x = self.x + addend.x;
        self.y = self.y + addend.y;
        self.z = self.z + addend.z;

    }

    pub fn subtract(&mut self, subtrahend: &Point3D<F>) {
        self.x = self.x - subtrahend.x;
        self.y = self.y - subtrahend.y;
        self.z = self.z - subtrahend.z;
    }

    pub fn dot_product(&mut self, multiplier: &Point3D<F>) -> F {
        self.x * multiplier.x + self.y * multiplier.y + self.z * multiplier.z
    }
}

impl<F: Float + Add<F> + Sub<F> + Mul<F> + Div<F>> Point3D<F> {
    pub fn new(x: F, y: F, z: F) -> Point3D<F> {
        Point3D { x, y, z }
    }

    pub fn origin() -> Point3D<F> {
        Point3D::new(F::zero(), F::zero(), F::zero())
    }

    pub fn new_add(left_addend: &Point3D<F>, right_addend: &Point3D<F>) -> Point3D<F> {
        Point3D {
            x: left_addend.x + right_addend.x,
            y: left_addend.y + right_addend.y,
            z: left_addend.z + right_addend.z,
        }
    }

    pub fn new_subtract(minuend: &Point3D<F>, subtrahend: &Point3D<F>) -> Point3D<F> {
        Point3D {
            x: minuend.x - subtrahend.x,
            y: minuend.y - subtrahend.y,
            z: minuend.z - subtrahend.z,
        }
    }

    pub fn new_mult_scalar(multiplicand: &Point3D<F>, multiplier: F) -> Point3D<F> {
        Point3D {
            x: multiplicand.x * multiplier,
            y: multiplicand.y * multiplier,
            z: multiplicand.z * multiplier,
        }
    }

    pub fn new_div_scalar(dividend: &Point3D<F>, divisor: F) -> Point3D<F> {
        Point3D {
            x: dividend.x / divisor,
            y: dividend.y / divisor,
            z: dividend.z / divisor,
        }
    }

    pub fn new_cross_product(left: &Point3D<F>, right: &Point3D<F>) -> Point3D<F> {
        Point3D {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x,
        }
    }
}

pub fn magnitude<F: Float>(p: &Point3D<F>) -> F {
    ((p.x*p.x) + (p.y*p.y) + (p.z*p.z)).sqrt()
}

pub fn distance_squared<F: Float>(left: &Point3D<F>, right: &Point3D<F>) -> F {
    let dx = left.x - right.x;
    let dy = left.y - right.y;
    let dz = left.z - right.z;
    dx * dx + dy * dy + dz * dz
}

pub fn distance<F: Float>(left: &Point3D<F>, right: &Point3D<F>) -> F {
    distance_squared(left, right).sqrt()
}

#[cfg(test)]
mod point3d_distance_tests {
    use crate::geometry::vector::p3d::Point3D;

    #[test]
    fn test_positive1() {
        let left = Point3D::new(2.0, 3.0, 4.0);
        let right = Point3D::new(5.0,7.0, 9.0);

        let dist = left.distance_to(&right);

        assert_eq!(7.0710678118654755, dist);
    }

    #[test]
    fn test_mixed1() {
        let left = Point3D::new(-3.0, 4.0, -2.0);
        let right = Point3D::new(1.0, -1.0, 3.0);

        let dist = left.distance_to(&right);

        assert_eq!(8.12403840463596, dist);
    }
}
