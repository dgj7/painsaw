//!
//! represents a handle to a graphics subsystem/api.
//!
//! for example, opengl or directx.
//!

use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::storage::g2d::Graph2D;
use crate::geometry::storage::g3d::Graph3D;
use crate::render::handle::info::RendererInfo;
use crate::render::handle::rssimpl::opengl::OpenGLHandle;
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::window::render::context::RendererContext;

#[derive(Clone)]
pub enum GraphicsSubSystem {
    OpenGL,
}

pub trait RenderingSubSystemHandle<F: Float + Add<F> + Sub<F>> {
    fn identify(&self) -> RendererInfo;
    
    fn initialize(&self, context: &RendererContext<F>);

    fn before_scene(&self, ccd: &Dimension2D<F>);

    fn render_2d(&self, graph: &Graph2D<F>, ccd: &Dimension2D<F>);

    fn render_3d(&self, graph: &Graph3D<F>);
}

pub fn grss_factory<F: Float + Add<F> + Sub<F>>(gss: GraphicsSubSystem) -> Box<dyn RenderingSubSystemHandle<F>> {
    match gss {
        GraphicsSubSystem::OpenGL => Box::new(OpenGLHandle {})
    }
}
