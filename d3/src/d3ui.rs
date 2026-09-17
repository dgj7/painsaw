use engine::graphics::camera::Camera;
use engine::graphics::storage::qt::attrib::align::Alignment;
use engine::graphics::storage::qt::attrib::layout::Layout;
use engine::graphics::storage::qt::attrib::sizing::Sizing;
use engine::graphics::storage::qt::panel::PanelBuilder;
use engine::graphics::storage::qt::view::{View, ViewBuilder};

pub(super) fn ui1(camera: &Camera) -> View {
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

pub(super) fn ui2(camera: &Camera) -> View {
    ViewBuilder::new()
        .with_vertical_sizing(Sizing::Exact { size: 350.0 })
        .with_horizontal_sizing(Sizing::Exact { size: 600.0 })
        .with_vertical_alignment(Alignment::Minimum)
        .with_horizontal_alignment(Alignment::Minimum)
        .with_client_dimensions(camera.screen.current_client_dimensions.clone())
        .with_panel(PanelBuilder::new()
            .with_padding(Sizing::Exact { size: 5.0 })
            .with_horizontal_sizing(Sizing::Exact { size: 200.0 })
            .with_vertical_sizing(Sizing::Exact { size: 200.0 })
            .with_layout(Layout::Horizontal)
            .build()
            .expect("panel: failed"))

        .build()
        .expect("view: failed")
}

pub(super) fn ui3(camera: &Camera) -> View {
    ViewBuilder::new()
        .with_vertical_sizing(Sizing::Percentage { percent: 0.36 })
        .with_horizontal_sizing(Sizing::RemainingSpace {})
        .with_vertical_alignment(Alignment::Maximum)
        .with_horizontal_alignment(Alignment::Maximum)
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
