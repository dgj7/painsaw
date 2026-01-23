use crate::graphics::geometry::line::Lines2D;
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::graphics::image::t2d::Texture2D;
use crate::graphics::geometry::point::Points2D;

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
