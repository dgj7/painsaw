use crate::geometry::primitive::v2d::Vertex2D;

///
/// anything that can be clicked.
///
/// this generally includes all ui elements, even if they don't respond to a click.
///
pub trait Clickable {
    fn click(&mut self, point: &Vertex2D);
}
