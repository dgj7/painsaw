use crate::input::mouse::min::MouseInputName;
use crate::window::context::RendererContext;
use std::sync::Arc;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

///
/// handle mouse inputs.
///
pub fn handle_mouse_change(handler: Arc<dyn MouseHandler>, name: &MouseInputName, context: &mut RendererContext) {
    match name {
        MouseInputName::MouseLeftButton => handler.handle_left_click(context),
        MouseInputName::MouseRightButton => handler.handle_right_click(context),
        MouseInputName::MouseScroll => {}
        MouseInputName::MouseMove => handler.handle_mouse_move(context),
    }
}


///
/// handle mouse changes.
///
pub trait MouseHandler {
    ///
    /// handle mouse move.
    ///
    fn handle_mouse_move(&self, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("MouseMove; frame {}", context.frame_count)));
    }

    fn handle_left_click(&self, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("LeftClick; frame {}", context.frame_count)));
    }

    fn handle_right_click(&self, context: &mut RendererContext) {
        log(LogLevel::Trace, &|| String::from(format!("RightClick; frame {}", context.frame_count)));
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
