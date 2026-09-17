use crate::d3::Demo3;
use engine::geometry::primitive::v2d::Vertex2D;
use engine::graphics::camera::Camera;
use engine::graphics::storage::gxd::Models;
use engine::graphics::storage::qt::attrib::layout::Layout;
use engine::graphics::storage::qt::panel::PanelBuilder;
use engine::graphics::storage::qt::attrib::sizing::Sizing;
use engine::graphics::storage::qt::view::ViewBuilder;
use engine::input::UserInput;
use engine::support::timing::EngineTiming;
use engine::WorldController;
use std::sync::{Arc, Mutex};

impl WorldController for Demo3 {
    fn initialize_world_helper(&self, camera: &Camera, models: &mut Models) {
        /* 2d */
        models.ui.add(1, ViewBuilder::new()
            .with_vertical_sizing(Sizing::Exact { size: 300.0 })
            .with_horizontal_sizing(Sizing::Exact { size: 300.0 })
            .with_client_dimensions(camera.screen.current_client_dimensions.clone())
            .with_panel(PanelBuilder::new()
                .with_padding(Sizing::Exact { size: 5.0 })
                .with_layout(Layout::Vertical)
                .with_horizontal_sizing(Sizing::Exact { size: 200.0 })
                .with_vertical_sizing(Sizing::Exact { size: 200.0 })
                .build()
                .expect("panel: failed"))
            .build()
            .expect("view: failed"));

        /* enable the first ui */
        models.ui.activate(1);
    }

    fn update_world_helper(&self, _input: Arc<Mutex<UserInput>>, _camera: &Camera, _timing: &mut EngineTiming, _models: &mut Models) {}
}
