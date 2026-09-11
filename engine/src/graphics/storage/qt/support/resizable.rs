use crate::geometry::dim::Dimension2D;

pub trait Resizable {
    ///
    /// resize the element.
    ///
    /// likely called when the window is resized, but not necessarily.
    ///
    fn resize(&mut self, dim: &Dimension2D);
}
