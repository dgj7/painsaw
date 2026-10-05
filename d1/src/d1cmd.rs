use crate::d1::Demo1;
use engine::config::EngineConfig;
use engine::game::Game;
use engine::geometry::orient::movement::spectator::SpectatorMovementStrategy;
use engine::graphics::camera::Camera;
use engine::support::timing::EngineTiming;

pub(crate) enum Command {
    CameraMoveForward,
    CameraStrafeLeft,
    CameraMoveBackward,
    CameraStrafeRight,
}

impl SpectatorMovementStrategy for Demo1 {}

pub(crate) fn handle_command(
    game: &Demo1,
    command: &Command,
    ec: &EngineConfig,
    camera: &mut Camera,
    et: &EngineTiming,
) {
    match command {
        Command::CameraMoveForward => {
            if !game.is_menu() {
                game.move_forward(ec, camera, et)
            }
        }
        Command::CameraStrafeLeft => {
            if !game.is_menu() {
                game.move_left(ec, camera, et)
            }
        }
        Command::CameraMoveBackward => {
            if !game.is_menu() {
                game.move_backward(ec, camera, et)
            }
        }
        Command::CameraStrafeRight => {
            if !game.is_menu() {
                game.move_right(ec, camera, et)
            }
        }
    }
}
