//!
//! represents a handle to a graphics subsystem/api.
//!
//! for example, opengl or directx.
//!

use crate::geometry::dim::d2d::Dimension2D;
use crate::geometry::line::ls2d::Lines2D;
use crate::geometry::line::ls3d::Lines3D;
use crate::geometry::vector::ps2d::Points2D;
use crate::geometry::vector::ps3d::Points3D;
use crate::graphics::model::renderer_info::RendererInfo;
use crate::graphics::subsystem::opengl::OpenGLHandle;
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::fileio::image::tex::t2d::Texture2D;

pub mod opengl;

#[derive(Clone)]
pub enum GraphicsSubSystem {
    OpenGL { pipeline: OpenGLPipeline },
}

#[derive(Clone)]
pub enum OpenGLPipeline {
    FixedFunction,
    Shaders,
}

pub trait RenderingSubSystemHandle<F: Float + Add<F> + Sub<F>> {
    fn identify(&self) -> Option<RendererInfo>;
    fn initialize(&self);
    fn before_scene(&self, ccd: &Dimension2D<f32>);
    fn prepare_2d(&self, ccd: &Dimension2D<f32>);
    fn prepare_3d(&self);
    fn render_2d_points(&self, points: &Points2D<F>);
    fn render_2d_lines(&self, lines: &Lines2D<F>);
    fn render_2d_textures(&self, textures: &Texture2D<F>);
    fn render_3d_points(&self, points: &Points3D<F>);
    fn render_3d_lines(&self, lines: &Lines3D<F>);
}

pub fn grss_factory<F: Float + Add<F> + Sub<F>>(gss: GraphicsSubSystem) -> Box<dyn RenderingSubSystemHandle<F>> {
    match gss {
        GraphicsSubSystem::OpenGL { pipeline: pl } => Box::new(OpenGLHandle { pipeline: pl })
    }
}
