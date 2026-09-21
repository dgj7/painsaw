use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::storage::ui::UIManager;

pub struct Models {
    pub ui: UIManager<u32>,
    pub g2d: Graph2D,
    pub g3d: Graph3D,
}

impl Models {
    pub fn new(ui: UIManager<u32>, g2d: Graph2D, g3d: Graph3D) -> Models {
        Models {
            ui,
            g2d,
            g3d,
        }
    }
}
