use engine::config::input_config::mc::MouseHandler;
use engine::config::EngineConfig;
use engine::graphics::camera::Camera;
use engine::input::mouse::ms::MouseState;
use engine::input::screen::ScreenState;
use engine::support::timing::EngineTiming;

pub(crate) struct MouseInputs {}

impl MouseHandler for MouseInputs {
    fn handle_mouse_move(&self, state: &mut MouseState, camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming, _screen: &mut ScreenState) {
        //let center = &screen.window_center;
        //let dx = MouseState::change_x(state);
        //let dy = MouseState::change_y(state);

        camera.orientation.look(state, &config, &timing);
/*
        state.enabled = false;
        screen.enabled = false;

        move_cursor(center);

        state.enabled = true;
        screen.enabled = true;

        state.current.x = center.x as i32;
        state.current.y = center.y as i32;
        state.previous.x = (center.x - dx) as i32;
        state.previous.y = (center.y - dy) as i32;*/
    }
}
