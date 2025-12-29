use std::ops::{Add, Sub};
use num_traits::Float;
use crate::geometry::line::l2d::Line2D;
use crate::render::model::color::Color;

#[derive(Clone)]
pub struct Lines2D<F: Float + Add<F> + Sub<F>> {
    pub lines: Vec<Line2D<F>>,
    pub color: Color,
    pub thickness: F,
}

impl<F: Float + Add<F> + Sub<F>> Lines2D<F> {
    pub fn new(lines: Vec<Line2D<F>>, color: Color, thickness: F) -> Lines2D<F> {
        Lines2D {
            lines,
            color,
            thickness,
        }
    }
}
