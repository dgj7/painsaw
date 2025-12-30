//!
//! represents a handle to a graphics subsystem/api.
//!
//! for example, opengl or directx.
//!

use std::ops::{Add, Sub};
use num_traits::Float;
use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::storage::g2d::Graph2D;
use crate::geometry::storage::g3d::Graph3D;
use crate::graphics::model::renderer_info::RendererInfo;
use crate::graphics::subsystem::opengl::OpenGLHandle;
use crate::window::wc::context::RendererContext;

pub mod opengl;

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
