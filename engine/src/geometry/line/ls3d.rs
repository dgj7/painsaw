use std::ops::{Add, Sub};
use num_traits::Float;
use crate::geometry::line::l3d::Line3D;
use crate::wc::model::color::Color;

#[derive(Clone)]
pub struct Lines3D<F: Float + Add<F> + Sub<F>> {
    pub lines: Vec<Line3D<F>>,
    pub color: Color,
    pub thickness: F,
}

impl<F: Float + Add<F> + Sub<F>> Lines3D<F> {
    pub fn new(lines: Vec<Line3D<F>>, color: Color, thickness: F) -> Lines3D<F> {
        Lines3D {
            lines,
            color,
            thickness,
        }
    }
}