use crate::fileio::image::raw::RawImage;
use crate::geometry::vector::p2d::Point2D;
use num_traits::Float;

pub struct Texture2D<F: Float> {
    pub id: u32,
    pub image: RawImage,
    pub world_pos: Point2D<F>,
}

impl<F: Float> Texture2D<F> {
    pub fn new(image: RawImage, world_pos: Point2D<F>) -> Texture2D<F> {
        Texture2D {
            id: 0,
            image,
            world_pos,
        }
    }
}
