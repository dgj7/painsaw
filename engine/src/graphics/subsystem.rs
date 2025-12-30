//!
//! represents a handle to a graphics subsystem/api.
//!
//! for example, opengl or directx.
//!

use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::storage::g2d::Graph2D;
use crate::geometry::storage::g3d::Graph3D;
use crate::graphics::model::renderer_info::RendererInfo;
use crate::graphics::subsystem::opengl::OpenGLHandle;
use num_traits::Float;
use std::ops::{Add, Sub};

pub mod opengl;

#[derive(Clone)]
pub enum GraphicsSubSystem {
    OpenGL,
}

pub trait RenderingSubSystemHandle<F: Float + Add<F> + Sub<F>> {
    fn identify(&self) -> RendererInfo;

    fn initialize(&self);

    fn before_scene(&self, ccd: &Dimension2D<f32>);

    fn prepare_2d(&self, ccd: &Dimension2D<f32>);

    fn render_2d(&self, graph: &Graph2D<F>);

    fn prepare_3d(&self);

    fn render_3d(&self, graph: &Graph3D<F>);
}

pub fn grss_factory<F: Float + Add<F> + Sub<F>>(gss: GraphicsSubSystem) -> Box<dyn RenderingSubSystemHandle<F>> {
    match gss {
        GraphicsSubSystem::OpenGL => Box::new(OpenGLHandle {})
    }
}
