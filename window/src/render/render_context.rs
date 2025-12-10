
pub struct RendererContext {
    pub first_frame_rendered: bool,
    pub frame_count: u128,
}

impl RendererContext {
    pub(crate) fn new() -> RendererContext {
        RendererContext { 
            first_frame_rendered: false, 
            frame_count: 0,
        }
    }
}
