use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::rect::Rectangle2D;

#[derive(Clone, Debug)]
pub struct Dimension2D {
    pub height: f32,
    pub width: f32,
}

impl Dimension2D {
    pub fn new(height: f32, width: f32) -> Dimension2D {
        Dimension2D { height, width }
    }

    pub fn is_equal(&self, other: &Dimension2D) -> bool {
        self.height == other.height && self.width == other.width
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0.0 && self.height == 0.0
    }
    
    pub fn to_rectangle(&self) -> Rectangle2D {
        Rectangle2D { 
            origin: Vertex2D { x: 0.0, y: 0.0 },
            antipode: Vertex2D { x: self.width, y: self.height },
        }
    }
}
