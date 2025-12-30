use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::storage::g2d::Graph2D;
use crate::geometry::storage::g3d::Graph3D;
use crate::graphics::subsystem::{grss_factory, GraphicsSubSystem, RenderingSubSystemHandle};
use num_traits::Float;
use std::ops::{Add, Sub};

pub mod model;
pub mod subsystem;

///
/// Graphics rendering intermediary.
///
/// This system separates/abstracts the concrete graphics rendering subsystem
/// from both the operating system and the geometry systems.
pub(crate) struct GraphicsIntermediary<F: Float + Add<F> + Sub<F>> {
    subsystem: Box<dyn RenderingSubSystemHandle<F>>,
}

impl<F: Float + Add<F> + Sub<F>> GraphicsIntermediary<F> {
    pub(crate) fn new(grss: GraphicsSubSystem) -> GraphicsIntermediary<F> {
        GraphicsIntermediary {
            subsystem: grss_factory(grss),
        }
    }
    
    pub(crate) fn initialize(&self) {
        self.subsystem.initialize();
    }
    
    pub(crate) fn before_scene(&self, ccd: &Dimension2D<f32>) {
        self.subsystem.before_scene(ccd);
    }
    
    pub(crate) fn prepare_2d(&self, ccd: &Dimension2D<f32>) {
        self.subsystem.prepare_2d(ccd);
    }
    
    pub(crate) fn render_2d(&self, g2d: &Graph2D<F>) {
        self.subsystem.render_2d(g2d);
    }
    
    pub(crate) fn prepare_3d(&self) {
        self.subsystem.prepare_3d();
    }
    
    pub(crate) fn render_3d(&self, g3d: &Graph3D<F>) {
        self.subsystem.render_3d(g3d);
    }
}
