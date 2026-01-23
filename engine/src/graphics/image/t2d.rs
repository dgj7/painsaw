use crate::graphics::geometry::point::p2d::Point2D;
use num_traits::Float;
use crate::graphics::image::RawImage;

pub struct Texture2D<F: Float> {
    pub initialized: bool,
    pub id: u32,
    pub image: RawImage,
    pub world_pos: Point2D<F>,
    pub scale: F,
    
    pub replacement: Option<RawImage>,
}

impl<F: Float> Texture2D<F> {
    pub fn new(image: RawImage, world_pos: Point2D<F>, scale: F) -> Texture2D<F> {
        Texture2D {
            initialized: false,
            id: 0,
            image,
            world_pos,
            scale,
            
            replacement: None,
        }
    }
}
