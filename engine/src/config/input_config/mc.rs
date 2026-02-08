use crate::input::r#in::InputName;
use crate::logger::log;
use crate::logger::log_level::LogLevel;
use crate::window::context::RendererContext;
use std::sync::Arc;

///
/// handle mouse inputs.
///
pub fn handle_mouse_change(handler: Arc<dyn MouseHandler>, name: &InputName, context: &mut RendererContext) {
    match name {
        InputName::KeyEscape => {}
        InputName::KeyA => {}
        InputName::KeyD => {}
        InputName::KeyG => {}
        InputName::KeyM => {}
        InputName::KeyS => {}
        InputName::KeyW => {}
        InputName::MouseLeftButton => {}
        InputName::MouseRightButton => {}
        InputName::MouseScroll => {}
        InputName::MouseMove { .. } => handler.handle_mouse_move(name, context),
    }
}


///
/// handle mouse changes.
///
pub trait MouseHandler {
    ///
    /// handle mouse move.
    ///
    fn handle_mouse_move(&self, name: &InputName, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("MouseMove: {:?},{:?}", name, context.frame_count)));
    }
}

///
/// default mouse handler.
/// 
pub struct DefaultMouseHandler {}

///
/// implement nothing for default mouse handler.
/// 
impl MouseHandler for DefaultMouseHandler {}
