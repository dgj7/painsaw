use crate::geometry::line::ls2d::Lines2D;
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::image::t2d::Texture2D;
use crate::geometry::point::ps2d::Points2D;

pub struct Model2D<F: Float + Add<F> + Sub<F>> {
    pub points: Vec<Points2D<F>>,
    pub lines: Vec<Lines2D<F>>,
    pub textures: Vec<Texture2D<F>>,
}

impl<F: Float + Add<F> + Sub<F>> Model2D<F> {
    pub fn new(points: Vec<Points2D<F>>, lines: Vec<Lines2D<F>>, textures: Vec<Texture2D<F>>) -> Model2D<F> {
        Model2D {
            points,
            lines,
            textures,
        }
    }
}
