use crate::geometry::line::ls2d::Lines2D;
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::geometry::vector::ps2d::Points2D;

pub struct Model2D<F: Float + Add<F> + Sub<F>> {
    pub points: Vec<Points2D<F>>,
    pub lines: Vec<Lines2D<F>>,
}

impl<F: Float + Add<F> + Sub<F>> Model2D<F> {
    pub fn new(points: Vec<Points2D<F>>, lines: Vec<Lines2D<F>>) -> Model2D<F> {
        Model2D {
            points,
            lines,
        }
    }
}
