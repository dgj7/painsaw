use engine::game::{Game, GameState};
use crate::d3::Demo3;

impl Game for Demo3 {
    fn game_state(&self) -> &GameState {
        &self.state
    }

    fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.state
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
