use crate::geometry::rect::Rectangle2D;
use crate::graphics::storage::g2d::m2d::Model2D;

pub trait Assembled {
    fn reassemble(&self, debug: bool, model: &mut Model2D, rectangle: &Rectangle2D);
}
