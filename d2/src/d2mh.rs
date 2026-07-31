use engine::config::input_config::mc::MouseHandler;
use engine::input::mouse::ms::MouseState;
use engine::PainsawContext;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, _state: &mut MouseState, _context: &mut PainsawContext) {
        //log(LogLevel::Debug, &|| String::from(format!("mouse-move: ({},{})", x, y)));
    }
}
