use crate::render::handle::def::RenderingSubSystemHandle;
use crate::render::handle::info::RendererInfo;

pub struct OpenGLHandle {
    pub info: Option<RendererInfo>,
}

impl RenderingSubSystemHandle for OpenGLHandle {
    fn identify(&self) -> RendererInfo {
        todo!()
    }
}
