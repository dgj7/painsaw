use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::storage::qt::panel::Panel;

///
/// [Panel] sizer specification, to determine how elements are laid out within.
///
pub enum Layout {
    Horizontal,
    Vertical,
    Grid,
}

impl Layout {
    pub fn determine_next(&self, panel: &Panel) -> (Vertex2D, Dimension2D) {
        match self {
            Layout::Horizontal => horizontal(panel),
            Layout::Vertical => vertical(panel),
            Layout::Grid => grid(panel),
        }
    }
}

fn horizontal(panel: &Panel) -> (Vertex2D, Dimension2D) {
    todo!()
}

fn vertical(panel: &Panel) -> (Vertex2D, Dimension2D) {
    todo!()
}

fn grid(panel: &Panel) -> (Vertex2D, Dimension2D) {
    todo!()
}
