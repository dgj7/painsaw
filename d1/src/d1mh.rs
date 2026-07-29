use engine::config::input_config::mc::MouseHandler;
use engine::input::mouse::ms::MouseState;
use engine::window::context::RendererContext;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, _state: &MouseState, _context: &mut RendererContext) {
        //log(LogLevel::Debug, &|| String::from(format!("mouse-move: ({},{})", x, y)));
    }
}
