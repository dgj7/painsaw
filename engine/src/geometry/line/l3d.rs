use crate::geometry::vector::p3d::Point3D;
use num_traits::Float;
use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct Line3D<F: Float + Add<F> + Sub<F>> {
    pub a: Point3D<F>,
    pub b: Point3D<F>,
}

impl<F: Float + Add<F> + Sub<F>> Line3D<F> {
    pub fn new(a: Point3D<F>, b: Point3D<F>) -> Line3D<F> {
        Line3D { a, b }
    }
}
