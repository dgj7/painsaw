use engine::config::input_config::mc::MouseHandler;
use engine::input::mouse::ms::MouseState;
use engine::PainsawContext;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, state: &mut MouseState, context: &mut PainsawContext) {
        context.camera.orientation.look(state, &context.config, &context.timing);
        //center_mouse()
    }
}
