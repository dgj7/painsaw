use std::ops::{Add, Sub};
use num_traits::Float;
use crate::graphics::geometry::primitive::line::l2d::Line2D;
use crate::graphics::geometry::primitive::line::l3d::Line3D;
use crate::graphics::color::Color;

pub mod l3d;
pub mod l2d;

#[derive(Clone)]
pub struct Lines2D<F: Float + Add<F> + Sub<F>> {
    pub lines: Vec<Line2D<F>>,
    pub color: Color,
    pub thickness: F,
}

#[derive(Clone)]
pub struct Lines3D<F: Float + Add<F> + Sub<F>> {
    pub lines: Vec<Line3D<F>>,
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

impl<F: Float + Add<F> + Sub<F>> Lines3D<F> {
    pub fn new(lines: Vec<Line3D<F>>, color: Color, thickness: F) -> Lines3D<F> {
        Lines3D {
            lines,
            color,
            thickness,
        }
    }
}
