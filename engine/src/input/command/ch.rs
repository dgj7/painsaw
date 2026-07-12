use crate::input::command::ce::Command;

// todo: remove this unused ----v
#[allow(unused)]
pub trait CommandHandler {
    fn handle(&self, cmd : Command) {
        match cmd {
            Command::CameraMoveForward => self.camera_move_forward(),
            Command::CameraMoveBackward => self.camera_move_backward(),
            Command::CameraStrafeLeft => self.camera_strafe_left(),
            Command::CameraStrafeRight => self.camera_strafe_right(),
            Command::ToggleConsole => self.toggle_console(),
            Command::QuitGame => self.quit_game(),
        }
    }

    fn camera_move_forward(&self);

    fn camera_move_backward(&self);

    fn camera_strafe_left(&self);

    fn camera_strafe_right(&self);

    fn toggle_console(&self);

    fn quit_game(&self);
}
