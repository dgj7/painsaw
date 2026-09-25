use engine::game::GameState;

pub struct Demo3 {
    pub state: GameState,
}

impl Demo3 {
    pub(crate) fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }
}
