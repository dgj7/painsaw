use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::storage::ui::UIManager;

pub mod g2d;
pub mod g3d;
pub mod ui;

pub struct Models {
    pub ui: UIManager,
    pub g2d: Graph2D,
    pub g3d: Graph3D,
}

impl Models {
    pub fn new(ui: UIManager, g2d: Graph2D, g3d: Graph3D) -> Models {
        Models {
            ui,
            g2d,
            g3d,
        }
    }
}
