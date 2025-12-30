use std::ops::{Add, Sub};
use num_traits::Float;
use crate::geometry::vector::p2d::Point2D;
use crate::graphics::model::color::Color;

#[derive(Clone)]
pub struct Points2D<F: Float + Add<F> + Sub<F>> {
    pub points: Vec<Point2D<F>>,
    pub color: Color,
    pub thickness: F,
}

impl<F: Float + Add<F> + Sub<F>> Points2D<F> {
    pub fn new(points: Vec<Point2D<F>>, color: Color, thickness: F) -> Points2D<F> {
        Points2D {
            points,
            color,
            thickness,
        }
    }
}
