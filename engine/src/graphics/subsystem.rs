//!
//! represents a handle to a graphics subsystem/api.
//!
//! for example, opengl or directx.
//!

use crate::graphics::camera::Camera;
use crate::graphics::storage::g2d::Graph2D;
use crate::graphics::storage::g3d::Graph3D;
use crate::graphics::subsystem::opengl::OpenGLHandle;
use crate::window::context::RendererContext;

pub mod opengl;

#[derive(Clone)]
pub enum GraphicsSubSystem {
    OpenGL { pipeline: OpenGLPipeline },
}

#[derive(Clone)]
pub enum OpenGLPipeline {
    FixedFunction,
    ProgrammableShader,
}

#[derive(Debug)]
pub struct RendererInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub vendor: Option<String>,
    pub device: Option<String>,
}

pub trait RenderingSubSystemHandle {
    fn identify(&self) -> Option<RendererInfo>;

    fn initialize(&self, g2d: &mut Graph2D, g3d: &mut Graph3D);

    fn resize(&self, context: &RendererContext);

    fn before_scene(&self, camera: &Camera);

    fn prepare_2d(&self, camera: &Camera, g2d: &mut Graph2D);
    fn render_2d(&self, g2d: &mut Graph2D);
    fn after_2d(&self);

    fn prepare_3d(&self, context: &RendererContext);
    fn render_3d(&self, g3d: &mut Graph3D);
    fn after_3d(&self, context: &RendererContext);
}

pub fn grss_factory(gss: GraphicsSubSystem) -> Box<dyn RenderingSubSystemHandle> {
    match gss {
        GraphicsSubSystem::OpenGL { pipeline: pl } => Box::new(OpenGLHandle { pipeline: pl }),
    }
}
