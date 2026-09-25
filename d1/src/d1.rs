use engine::game::GameState;

#[derive(Clone)]
pub struct Demo1 {
    pub state: GameState,
}

impl Demo1 {
    pub(crate) fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }
}
