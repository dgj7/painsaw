use crate::input::min::MouseInputName;
use crate::window::context::RendererContext;
use std::sync::Arc;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

///
/// handle mouse inputs.
///
pub fn handle_mouse_change(handler: Arc<dyn MouseHandler>, name: &MouseInputName, context: &mut RendererContext) {
    match name {
        MouseInputName::MouseLeftButton => {}
        MouseInputName::MouseRightButton => {}
        MouseInputName::MouseScroll => {}
        MouseInputName::MouseMove { x, y } => handler.handle_mouse_move(*x, *y, context),
    }
}


///
/// handle mouse changes.
///
pub trait MouseHandler {
    ///
    /// handle mouse move.
    ///
    fn handle_mouse_move(&self, x: i32, y: i32, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("MouseMove ({},{}); frame {}", x, y, context.frame_count)));
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
