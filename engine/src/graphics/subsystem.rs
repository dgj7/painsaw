//!
//! represents a handle to a graphics subsystem/api.
//!
//! for example, opengl or directx.
//!

use crate::graphics::image::t2d::Texture2D;
use crate::graphics::geometry::line::Lines2D;
use crate::graphics::geometry::line::Lines3D;
use crate::graphics::model::g2d::Graph2D;
use crate::graphics::model::g3d::Graph3D;
use crate::graphics::geometry::point::Points2D;
use crate::graphics::geometry::point::Points3D;
use crate::graphics::model::renderer_info::RendererInfo;
use crate::graphics::subsystem::opengl::OpenGLHandle;
use num_traits::Float;
use std::ops::{Add, Sub};
use crate::graphics::camera::Camera;
use crate::window::context::RendererContext;

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

    fn initialize(&self, g2d: &mut Graph2D<F>, g3d: &mut Graph3D<F>);
    fn initialize_texture_2d(&self, texture: &mut Texture2D<F>);
    fn update_texture_2d(&self, texture: &mut Texture2D<F>);

    fn resize(&self, context: &RendererContext<F>);

    fn before_scene(&self, camera: &Camera);

    fn prepare_2d(&self, camera: &Camera);
    fn after_2d(&self);

    fn prepare_3d(&self, context: &RendererContext<F>);
    fn after_3d(&self);

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
