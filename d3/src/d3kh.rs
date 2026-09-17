use engine::config::EngineConfig;
use engine::config::input_config::kc::KeyHandler;
use engine::graphics::camera::Camera;
use engine::graphics::storage::gxd::Models;
use engine::input::keyboard::kin::KeyInputName;
use engine::input::keyboard::ks::KeyState;
use engine::support::timing::EngineTiming;
use crate::d3::Demo3;

impl KeyHandler for Demo3 {
    fn handle_escape_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.deactivate();
    }

    fn handle_1_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(1);
    }

    fn handle_2_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(2);
    }

    fn handle_3_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(3);
    }

    fn handle_4_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(4);
    }

    fn handle_5_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(5);
    }

    fn handle_6_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(6);
    }

    fn handle_7_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(7);
    }

    fn handle_8_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(8);
    }

    fn handle_9_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(9);
    }

    fn handle_0_key_change(&mut self, _name: &KeyInputName, _state: &mut KeyState, _config: &EngineConfig, _camera: &mut Camera, _timing: &EngineTiming, models: &mut Models) {
        models.ui.activate(0);
    }
}
