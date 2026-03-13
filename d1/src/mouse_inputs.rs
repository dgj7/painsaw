use engine::config::input_config::mc::MouseHandler;
use engine::window::context::RendererContext;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, _x: i32, _y: i32, _context: &mut RendererContext) {
        //log(LogLevel::Debug, &|| String::from(format!("mouse-move: ({},{})", x, y)));
    }
}
