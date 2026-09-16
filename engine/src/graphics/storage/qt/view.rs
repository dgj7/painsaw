use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::panel::Panel;
use crate::graphics::storage::qt::sizing::Sizing;
use crate::input::screen::ScreenState;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub struct View {
    /* the main panel */
    pub panel: Panel,

    /* a renderable representation of the screen and it's main panel */
    pub model: Model2D,

    /* information relating to sizing of the screen and it's panel */
    pub origin: Vertex2D,
    pub vertical: Sizing,
    pub horizontal: Sizing,
}

impl View {
    pub fn resize(&mut self, screen: &ScreenState) {
        let mut model = Model2D::new(vec!(), vec!(), true);
        let antipode = compute_antipode(&self.vertical, &self.horizontal, &screen.current_client_dimensions);
        
        self.panel.reassemble(&mut model, &self.origin, &antipode);
        
        self.model = model;
    }
}

fn compute_antipode(vertical: &Sizing, horizontal: &Sizing, client: &Dimension2D) -> Vertex2D {
    let x = horizontal.screen_dimension_to_actual(client.width);
    let y = vertical.screen_dimension_to_actual(client.height);
    Vertex2D { x, y }
}

pub struct ViewBuilder {
    the_panel: Option<Panel>,
    the_origin: Option<Vertex2D>,
    the_vertical: Option<Sizing>,
    the_horizontal: Option<Sizing>,
    the_client_dimensions: Option<Dimension2D>,
}

impl ViewBuilder {
    pub fn new() -> ViewBuilder {
        ViewBuilder {
            the_panel: None,
            the_origin: None,
            the_vertical: None,
            the_horizontal: None,
            the_client_dimensions: None,
        }
    }

    pub fn with_panel(mut self, panel: Panel) -> ViewBuilder {
        self.the_panel = Some(panel);
        self
    }

    pub fn with_origin(mut self, origin: Vertex2D) -> ViewBuilder {
        self.the_origin = Some(origin);
        self
    }

    pub fn with_vertical(mut self, vertical: Sizing) -> ViewBuilder {
        self.the_vertical = Some(vertical);
        self
    }

    pub fn with_horizontal(mut self, horizontal: Sizing) -> ViewBuilder {
        self.the_horizontal = Some(horizontal);
        self
    }
    
    pub fn with_client_dimensions(mut self, dimensions: Dimension2D) -> ViewBuilder {
        self.the_client_dimensions = Some(dimensions);
        self
    }

    pub fn build(self) -> Option<View> {
        /* none if required fields are missing */
        if self.the_panel.is_none() || self.the_client_dimensions.is_none() {
            log(LogLevel::Warning, &|| String::from("panel or client dimensions not provided"));
            return None;
        }

        /* required fields */
        let panel = self.the_panel.unwrap();
        let client = self.the_client_dimensions.unwrap();

        /* pull optional fields, with replacements */
        let origin = self.the_origin.unwrap_or_else(|| Vertex2D::origin());
        let vertical = self.the_vertical.unwrap_or_else(|| Sizing::RemainingSpace {});
        let horizontal = self.the_horizontal.unwrap_or_else(|| Sizing::RemainingSpace {});

        /* compute the model */
        let mut model = Model2D::new(vec!(), vec!(), true);
        let antipode = compute_antipode(&vertical, &horizontal, &client);
        panel.reassemble(&mut model, &origin, &antipode);

        Some(View {
            panel,
            model,
            origin,
            vertical,
            horizontal,
        })
    }
}
