use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;

pub struct Models {
    pub g2d: Graph2D,
    pub g3d: Graph3D,
}

impl Models {
    pub fn new(g2d: Graph2D, g3d: Graph3D) -> Models {
        Models { g2d, g3d }
    }
}
