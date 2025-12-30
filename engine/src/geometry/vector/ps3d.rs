use std::ops::{Add, Sub};
use num_traits::Float;
use crate::geometry::vector::p3d::Point3D;
use crate::graphics::model::color::Color;

#[derive(Clone)]
pub struct Points3D<F: Float + Add<F> + Sub<F>> {
    pub points: Vec<Point3D<F>>,
    pub color: Color, 
    pub thickness: F,
}

impl<F: Float + Add<F> + Sub<F>> Points3D<F> {
    pub fn new(points: Vec<Point3D<F>>, color: Color, thickness: F) -> Points3D<F> {
        Points3D {
            points,
            color,
            thickness,
        }
    }
}
