use engine::config::input_config::mc::MouseHandler;
use engine::input::r#in::InputName;
use engine::logger::log;
use engine::logger::log_level::LogLevel;
use engine::window::context::RendererContext;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, name: &InputName, _context: &mut RendererContext) {
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
            InputName::MouseMove { x, y } => log(LogLevel::Debug, &|| String::from(format!("mouse-move: ({},{})", x, y))),
        }
    }
}
