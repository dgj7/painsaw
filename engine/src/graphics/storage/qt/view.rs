use crate::geometry::dim::Dimension2D;
use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::panel::Panel;
use crate::graphics::storage::qt::sr::SizingRequest;
use crate::input::screen::ScreenState;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub struct View {
    pub panel: Panel,           /* the main panel */
    pub model: Model2D,         /* a renderable representation of the screen and it's main panel */
    pub sizing: SizingRequest,  /* information relating to sizing of this view */
}

impl View {
    pub fn resize(&mut self, screen: &ScreenState) {
        let mut model = Model2D::new(vec!(), vec!(), true);
        let rectangle = self.sizing.client_to_sized_rectangle(&screen.current_client_dimensions);
        self.panel.reassemble(&mut model, &rectangle);
        self.model = model;
    }
}

pub struct ViewBuilder {
    the_panel: Option<Panel>,
    the_sizing: Option<SizingRequest>,
    the_client_dimensions: Option<Dimension2D>,
}

impl ViewBuilder {
    pub fn new() -> ViewBuilder {
        ViewBuilder {
            the_panel: None,
            the_sizing: None,
            the_client_dimensions: None,
        }
    }

    pub fn with_panel(mut self, panel: Panel) -> ViewBuilder {
        self.the_panel = Some(panel);
        self
    }

    pub fn with_sizing_request(mut self, sizing_request: SizingRequest) -> ViewBuilder {
        self.the_sizing = Some(sizing_request);
        self
    }
    
    pub fn with_client_dimensions(mut self, dimensions: Dimension2D) -> ViewBuilder {
        self.the_client_dimensions = Some(dimensions);
        self
    }

    pub fn build(self) -> Option<View> {
        /* none if required fields are missing */
        if self.the_panel.is_none() || self.the_client_dimensions.is_none() {
            log(LogLevel::Warning, &|| String::from("didn't provide panel or client"));
            return None;
        }

        /* required fields */
        let panel = self.the_panel.unwrap();
        let client = self.the_client_dimensions.unwrap();

        /* pull optional fields, with replacements */
        let sizing = self.the_sizing.unwrap_or_else(|| SizingRequest::default());

        /* compute the model */
        let mut model = Model2D::new(vec!(), vec!(), true);
        let rectangle = sizing.client_to_sized_rectangle(&client);
        panel.reassemble(&mut model, &rectangle);

        Some(View {
            panel,
            model,
            sizing,
        })
    }
}
