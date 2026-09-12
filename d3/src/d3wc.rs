use std::sync::{Arc, Mutex};
use engine::graphics::camera::Camera;
use engine::graphics::storage::gxd::Models;
use engine::input::UserInput;
use engine::support::timing::EngineTiming;
use engine::WorldController;
use crate::d3::Demo3;

impl WorldController for Demo3 {
    fn initialize_world_helper(&self, _camera: &Camera, _models: &mut Models) {}

    fn update_world_helper(&self, _input: Arc<Mutex<UserInput>>, _camera: &Camera, _timing: &mut EngineTiming, _models: &mut Models) {}
}
