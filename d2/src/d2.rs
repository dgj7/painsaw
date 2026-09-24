use engine::game::GameState;

pub struct Demo2 {
    pub state: GameState,
}

impl Demo2 {
    pub(crate) fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }
}
