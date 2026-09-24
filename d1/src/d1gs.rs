use engine::game::{Game, GameState};
use crate::d1::Demo1;

impl Game for Demo1 {
    fn game_state(&self) -> &GameState {
        &self.state
    }

    fn set_exit(&mut self, value: bool) {
        self.state.exiting = value;
    }

    fn set_paused(&mut self, value: bool) {
        self.state.paused = value;
    }

    fn set_menu(&mut self, value: bool) {
        self.state.menu = value;
    }
}
