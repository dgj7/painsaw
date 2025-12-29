//!
//! represents a handle to a graphics subsystem/api.
//!
//! for example, opengl or directx.
//!

use crate::render::handle::info::RendererInfo;
use crate::render::handle::rssimpl::opengl::OpenGLHandle;

pub enum GraphicsSubSystem {
    OpenGL,
}

pub trait RenderingSubSystemHandle {
    fn identify(&self) -> RendererInfo;
}

pub fn factory<T: RenderingSubSystemHandle>(gss: GraphicsSubSystem) -> Box<dyn RenderingSubSystemHandle> {
    match gss {
        GraphicsSubSystem::OpenGL => Box::new(OpenGLHandle { info: None })
    }
}
