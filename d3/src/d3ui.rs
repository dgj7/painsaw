use engine::graphics::camera::Camera;
use engine::graphics::storage::qt::attrib::align::Alignment;
use engine::graphics::storage::qt::attrib::layout::Layout;
use engine::graphics::storage::qt::attrib::sizing::Sizing;
use engine::graphics::storage::qt::panel::PanelBuilder;
use engine::graphics::storage::qt::view::{View, ViewBuilder};
use engine::graphics::storage::qt::widget::WidgetBuilder;

pub(super) fn ui1(camera: &Camera) -> View {
    ViewBuilder::new()
        .with_window_dimensions(camera.screen.current_client_dimensions.clone())
        .with_vertical_sizing(Sizing::Percentage { percent: 0.5 })
        .with_horizontal_sizing(Sizing::Percentage { percent: 0.2 })
        .with_vertical_alignment(Alignment::Center)
        .with_horizontal_alignment(Alignment::Center)
        .with_panel(PanelBuilder::new()
            .with_layout(Layout::Horizontal)
            .with_panel(PanelBuilder::new()
                .with_layout(Layout::Vertical)
                .build()
                .expect("VP1P1: failed"),
            Sizing::Percentage { percent: 0.1 })
            .with_panel(PanelBuilder::new()
                .with_layout(Layout::Vertical)
                .with_panel(PanelBuilder::new()
                                .with_layout(Layout::Horizontal)
                                .build()
                                .expect("x"),
                            Sizing::Percentage { percent: 0.1 })
                .with_widget(WidgetBuilder::new()
                                 .with_text("options")
                                 .build(), Sizing::Percentage { percent: 0.2 })
                .with_widget(WidgetBuilder::new()
                                 .with_text("exit game")
                                 .build(), Sizing::Percentage { percent: 0.2 })
                            .with_panel(PanelBuilder::new()
                                            .with_layout(Layout::Horizontal)
                                            .build()
                                            .expect("x"),
                                        Sizing::Percentage { percent: 0.20 })
                .with_widget(WidgetBuilder::new()
                                 .with_text("return to game")
                                 .build(), Sizing::Percentage { percent: 0.2 })
                .with_panel(PanelBuilder::new()
                                .with_layout(Layout::Horizontal)
                                .build()
                                .expect("x"),
                            Sizing::Percentage { percent: 0.095 })
                .build()
                .expect("VP1P2: failed"),
            Sizing::Percentage { percent: 0.8 })
            .with_panel(PanelBuilder::new()
                .with_layout(Layout::Horizontal)
                .build()
                .expect("VP1P3: failed"),
            Sizing::Percentage { percent: 0.1 })
            .build()
            .expect("V1P1: failed"))
        .build()
        .expect("view: failed")
}

pub(super) fn ui2(camera: &Camera) -> View {
    ViewBuilder::new()
        .with_window_dimensions(camera.screen.current_client_dimensions.clone())
        .with_vertical_sizing(Sizing::Percentage { percent: 0.9 })
        .with_horizontal_sizing(Sizing::Percentage { percent: 0.9 })
        .with_vertical_alignment(Alignment::Center)
        .with_horizontal_alignment(Alignment::Center)
        .with_panel(PanelBuilder::new()
            .with_layout(Layout::Vertical)
            .with_panel(PanelBuilder::new()
                            .with_layout(Layout::Horizontal)
                            .build()
                            .expect("VP1P1: failed"),
                        Sizing::Percentage { percent: 0.25 })
            .with_panel(PanelBuilder::new()
                            .with_layout(Layout::Horizontal)
                            .build()
                            .expect("VP1P2: failed"),
                        Sizing::Percentage { percent: 0.25 })
            .with_panel(PanelBuilder::new()
                            .with_layout(Layout::Horizontal)
                            .build()
                            .expect("VP1P3: failed"),
                        Sizing::Percentage { percent: 0.25 })
            .with_panel(PanelBuilder::new()
                            .with_layout(Layout::Horizontal)
                            .build()
                            .expect("VP1P4: failed"),
                        Sizing::Percentage { percent: 0.25 })
            .build()
            .expect("VP1: failed"))
        .build()
        .expect("view failed")
}

pub(super) fn ui3(camera: &Camera) -> View {
    ViewBuilder::new()
        .with_window_dimensions(camera.screen.current_client_dimensions.clone())
        .with_vertical_sizing(Sizing::Percentage { percent: 0.9 })
        .with_horizontal_sizing(Sizing::Percentage { percent: 0.9 })
        .with_vertical_alignment(Alignment::Center)
        .with_horizontal_alignment(Alignment::Center)
        .with_panel(PanelBuilder::new()
            .with_layout(Layout::Horizontal)
            .with_panel(PanelBuilder::new()
                            .with_layout(Layout::Horizontal)
                            .build()
                            .expect("VP1P1: failed"),
                        Sizing::Percentage { percent: 0.25 })
            .with_panel(PanelBuilder::new()
                            .with_layout(Layout::Horizontal)
                            .build()
                            .expect("VP1P2: failed"),
                        Sizing::Percentage { percent: 0.25 })
            .with_panel(PanelBuilder::new()
                            .with_layout(Layout::Horizontal)
                            .build()
                            .expect("VP1P3: failed"),
                        Sizing::Percentage { percent: 0.25 })
            .with_panel(PanelBuilder::new()
                            .with_layout(Layout::Horizontal)
                            .build()
                            .expect("VP1P4: failed"),
                        Sizing::Percentage { percent: 0.25 })
            .build()
            .expect("VP1: failed"))
        .build()
        .expect("view failed")
}

pub(super) fn ui4(camera: &Camera) -> View {
    ViewBuilder::new()
        .with_window_dimensions(camera.screen.current_client_dimensions.clone())
        .with_vertical_sizing(Sizing::Exact { size: 350.0 })
        .with_horizontal_sizing(Sizing::Exact { size: 600.0 })
        .with_vertical_alignment(Alignment::Minimum)
        .with_horizontal_alignment(Alignment::Minimum)
        .with_panel(PanelBuilder::new()
            .with_layout(Layout::Horizontal)
            .with_widget(WidgetBuilder::new()
                .build(),
                Sizing::Percentage { percent: 0.25 })
            .build()
            .expect("panel: failed"))
        .build()
        .expect("view: failed")
}

pub(super) fn ui5(camera: &Camera) -> View {
    ViewBuilder::new()
        .with_window_dimensions(camera.screen.current_client_dimensions.clone())
        .with_vertical_sizing(Sizing::Percentage { percent: 0.36 })
        .with_horizontal_sizing(Sizing::RemainingSpace {})
        .with_vertical_alignment(Alignment::Maximum)
        .with_horizontal_alignment(Alignment::Maximum)
        .with_panel(PanelBuilder::new()
            .with_layout(Layout::Vertical)
            .build()
            .expect("panel: failed"))
        .build()
        .expect("view: failed")
}
