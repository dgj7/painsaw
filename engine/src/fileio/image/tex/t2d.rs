use num_traits::Float;
use crate::fileio::image::raw::{Pixel, RawImage};
use crate::geometry::vector::p2d::Point2D;

pub struct Texture2D<F: Float> {
    id: u32,
    width: u32,
    height: u32,
    world_pos: Point2D<F>,
}

impl<F: Float> Texture2D<F> {
    pub fn new<P: Pixel>(image: &RawImage<P>, texture_id: u32, world_pos: Point2D<F>) -> Texture2D<F> {
        Texture2D {
            id: texture_id,
            width: image.width,
            height: image.height,
            world_pos,
        }
    }
}
