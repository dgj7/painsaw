///
/// General functionalities supported by the engine.
///
#[derive(Clone, Debug)]
pub enum Command {
    /* camera movement */
    CameraMoveForward,
    CameraMoveBackward,
    CameraStrafeLeft,
    CameraStrafeRight,

    /* developer commands */
    ToggleConsole,

    /* general ui commands */
    QuitGame,
}
