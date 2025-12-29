use crate::geometry::line::ls3d::Lines3D;
use num_traits::Float;
use std::ops::{Add, Sub};

pub struct Model3D<F: Float + Add<F> + Sub<F>> {
    pub lines: Vec<Lines3D<F>>,
}

impl<F: Float + Add<F> + Sub<F>> Model3D<F> {
    pub fn new(lines: Vec<Lines3D<F>>) -> Model3D<F> {
        Model3D {
            lines,
        }
    }
}
