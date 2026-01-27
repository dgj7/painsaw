use std::ops::{Add, Sub};
use num_traits::Float;
use crate::graphics::geometry::primitive::point::p2d::Point2D;
use crate::graphics::geometry::primitive::point::p3d::Point3D;
use crate::graphics::model::color::Color;

pub mod p3d;
pub mod p2d;

#[derive(Clone)]
pub struct Points2D<F: Float + Add<F> + Sub<F>> {
    pub points: Vec<Point2D<F>>,
    pub color: Color,
    pub thickness: F,
}

#[derive(Clone)]
pub struct Points3D<F: Float + Add<F> + Sub<F>> {
    pub points: Vec<Point3D<F>>,
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

impl<F: Float + Add<F> + Sub<F>> Points3D<F> {
    pub fn new(points: Vec<Point3D<F>>, color: Color, thickness: F) -> Points3D<F> {
        Points3D {
            points,
            color,
            thickness,
        }
    }
}
