use crate::d1::Demo1;
use engine::config::EngineConfig;
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

pub(crate) fn handle_command(command: &Command, camera: &mut Camera, ec: &EngineConfig, et: &EngineTiming) {
    match command {
        Command::CameraMoveForward => <Demo1 as SpectatorMovementStrategy>::move_forward(camera, ec, et),
        Command::CameraStrafeLeft => <Demo1 as SpectatorMovementStrategy>::move_left(camera, ec, et),
        Command::CameraMoveBackward => <Demo1 as SpectatorMovementStrategy>::move_backward(camera, ec, et),
        Command::CameraStrafeRight => <Demo1 as SpectatorMovementStrategy>::move_right(camera, ec, et),
    }
}
