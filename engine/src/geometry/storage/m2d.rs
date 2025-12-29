use crate::geometry::line::ls2d::Lines2D;
use num_traits::Float;
use std::ops::{Add, Sub};

pub struct Model2D<F: Float + Add<F> + Sub<F>> {
    pub lines: Vec<Lines2D<F>>,
}

impl<F: Float + Add<F> + Sub<F>> Model2D<F> {
    pub fn new(lines: Vec<Lines2D<F>>) -> Model2D<F> {
        Model2D {
            lines,
        }
    }
}
