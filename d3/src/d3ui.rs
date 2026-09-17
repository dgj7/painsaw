use engine::graphics::camera::Camera;
use engine::graphics::storage::qt::attrib::align::Alignment;
use engine::graphics::storage::qt::attrib::layout::Layout;
use engine::graphics::storage::qt::attrib::sizing::Sizing;
use engine::graphics::storage::qt::panel::PanelBuilder;
use engine::graphics::storage::qt::view::{View, ViewBuilder};

pub(crate) fn ui1(camera: &Camera) -> View {
    ViewBuilder::new()
        .with_vertical_sizing(Sizing::Percentage { percent: 0.9 })
        .with_horizontal_sizing(Sizing::Percentage { percent: 0.9 })
        .with_vertical_alignment(Alignment::Center)
        .with_horizontal_alignment(Alignment::Center)
        .with_client_dimensions(camera.screen.current_client_dimensions.clone())
        .with_panel(PanelBuilder::new()
            .with_padding(Sizing::Exact { size: 5.0 })
            .with_horizontal_sizing(Sizing::Exact { size: 200.0 })
            .with_vertical_sizing(Sizing::Exact { size: 200.0 })
            .with_layout(Layout::Vertical)
            .build()
            .expect("panel: failed"))
        .build()
        .expect("view: failed")
}
