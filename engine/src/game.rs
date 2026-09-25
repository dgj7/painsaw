///
/// various game states implemented by any game.
///
pub trait Game {
    ///
    /// get the game state.
    ///
    fn game_state(&self) -> &GameState;
    
    ///
    /// get the game state, mutable.
    /// 
    fn game_state_mut(&mut self) -> &mut GameState;

    ///
    /// determine if the application should close.
    ///
    fn is_exit(&self) -> bool {
        self.game_state().exiting
    }

    ///
    /// ask the application to close.
    ///
    fn set_exit(&mut self, value: bool);

    ///
    /// determine if the application is paused.
    ///
    fn is_paused(&self) -> bool {
        self.game_state().paused
    }

    ///
    /// ask the application to pause.
    ///
    fn set_paused(&mut self, value: bool);

    ///
    /// toggle the paused state.
    ///
    fn toggle_paused(&mut self) {
        self.set_paused(!self.is_paused());
    }

    ///
    /// determine if the application is displaying the main menu.
    ///
    fn is_menu(&self) -> bool {
        self.game_state().menu
    }

    ///
    /// ask the application to display the main menu.
    ///
    fn set_menu(&mut self, value: bool);

    ///
    /// toggle the menu state.
    ///
    fn toggle_menu(&mut self) {
        self.set_menu(!self.is_menu());
    }
}

///
/// various shared game states.
///
#[derive(Clone)]
pub struct GameState {
    pub exiting: bool,                              // window needs to close
    pub paused: bool,                               // animation stops
    pub menu: bool,                                 // menu is displayed; distinct from, and not necessarily, paused
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            exiting: false,
            paused: false,
            menu: false,
        }
    }
}
