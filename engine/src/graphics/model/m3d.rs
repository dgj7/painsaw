use crate::geometry::line::Lines3D;
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::geometry::point::Points3D;

pub struct Model3D<F: Float + Add<F> + Sub<F>> {
    pub points: Vec<Points3D<F>>,
    pub lines: Vec<Lines3D<F>>,
}

impl<F: Float + Add<F> + Sub<F>> Model3D<F> {
    pub fn new(points: Vec<Points3D<F>>, lines: Vec<Lines3D<F>>) -> Model3D<F> {
        Model3D {
            points,
            lines,
        }
    }
}
