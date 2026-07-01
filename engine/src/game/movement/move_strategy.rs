use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::support::timing::EngineTiming;

pub trait MovementStrategy {
    fn move_camera(&mut self, camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming);
}
