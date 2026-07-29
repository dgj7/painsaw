use engine::config::EngineConfig;
use engine::geometry::orient::Orientation;
use engine::support::timing::EngineTiming;

pub(crate) enum Command {
    CameraMoveForward,
    CameraStrafeLeft,
    CameraMoveBackward,
    CameraStrafeRight,
}

pub(crate) fn handle_command(command: &Command, orientation: &mut Orientation, ec: &EngineConfig, et: &EngineTiming) {
    match command {
        Command::CameraMoveForward => orientation.move_forward(&ec, &et),
        Command::CameraStrafeLeft => orientation.move_left(&ec, &et),
        Command::CameraMoveBackward => orientation.move_backward(&ec, &et),
        Command::CameraStrafeRight => orientation.move_right(&ec, &et),
    }
}
