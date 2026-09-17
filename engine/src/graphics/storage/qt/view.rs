use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::attrib::align::Alignment;
use crate::graphics::storage::qt::panel::Panel;
use crate::graphics::storage::qt::attrib::sizing::Sizing;
use crate::input::screen::ScreenState;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub struct View {
    /* the main panel */
    pub panel: Panel,

    /* a renderable representation of the screen and it's main panel */
    pub model: Model2D,

    /* information relating to sizing of the screen and it's panel */
    pub vertical_sizing: Sizing,
    pub horizontal_sizing: Sizing,

    pub vertical_alignment: Alignment,
    pub horizontal_alignment: Alignment,
}

impl View {
    pub fn resize(&mut self, screen: &ScreenState) {
        let mut model = Model2D::new(vec!(), vec!(), true);

        let origin = compute_origin(&self.vertical_sizing, &self.horizontal_sizing, &self.vertical_alignment, &self.horizontal_alignment, &screen.current_client_dimensions);
        let antipode = compute_antipode(&self.vertical_sizing, &self.horizontal_sizing, &self.vertical_alignment, &self.horizontal_alignment, &screen.current_client_dimensions);
        
        self.panel.reassemble(&mut model, &origin, &antipode);
        
        self.model = model;
    }
}

fn compute_origin(vertical_sizing: &Sizing, horizontal_sizing: &Sizing, _vertical_alignment: &Alignment, horizontal_alignment: &Alignment, client: &Dimension2D) -> Vertex2D {
    let width = horizontal_sizing.from_client_to_dimension(client.width);
    let height = vertical_sizing.from_client_to_dimension(client.height);
    let x = match horizontal_alignment {
        Alignment::Minimum => 0.0,
        Alignment::Center => (client.width / 2.0) - (width / 2.0),
        Alignment::Maximum => client.width - width,
    };
    let y = match horizontal_alignment {
        Alignment::Minimum => 0.0,
        Alignment::Center => (client.height / 2.0) - (height / 2.0),
        Alignment::Maximum => client.height - height,
    };
    Vertex2D { x, y }
}

fn compute_antipode(vertical_sizing: &Sizing, horizontal_sizing: &Sizing, vertical_alignment: &Alignment, _horizontal_alignment: &Alignment, client: &Dimension2D) -> Vertex2D {
    let width = horizontal_sizing.from_client_to_dimension(client.width);
    let height = vertical_sizing.from_client_to_dimension(client.height);
    let x = match vertical_alignment {
        Alignment::Minimum => width,
        Alignment::Center => (client.width / 2.0) + (width / 2.0),
        Alignment::Maximum => client.width,
    };
    let y = match vertical_alignment {
        Alignment::Minimum => client.height - height,
        Alignment::Center => (client.height / 2.0) + (height / 2.0),
        Alignment::Maximum => client.height,
    };
    Vertex2D { x, y }
}

pub struct ViewBuilder {
    the_panel: Option<Panel>,

    the_vertical_sizing: Option<Sizing>,
    the_horizontal_sizing: Option<Sizing>,

    the_vertical_alignment: Option<Alignment>,
    the_horizontal_alignment: Option<Alignment>,

    the_client_dimensions: Option<Dimension2D>,
}

impl ViewBuilder {
    pub fn new() -> ViewBuilder {
        ViewBuilder {
            the_panel: None,

            the_vertical_sizing: None,
            the_horizontal_sizing: None,

            the_vertical_alignment: None,
            the_horizontal_alignment: None,

            the_client_dimensions: None,
        }
    }

    pub fn with_panel(mut self, panel: Panel) -> ViewBuilder {
        self.the_panel = Some(panel);
        self
    }

    pub fn with_vertical_sizing(mut self, vertical: Sizing) -> ViewBuilder {
        self.the_vertical_sizing = Some(vertical);
        self
    }

    pub fn with_horizontal_sizing(mut self, horizontal: Sizing) -> ViewBuilder {
        self.the_horizontal_sizing = Some(horizontal);
        self
    }

    pub fn with_vertical_alignment(mut self, alignment: Alignment) -> ViewBuilder {
        self.the_vertical_alignment = Some(alignment);
        self
    }

    pub fn with_horizontal_alignment(mut self, alignment: Alignment) -> ViewBuilder {
        self.the_horizontal_alignment = Some(alignment);
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
        let vertical_sizing = self.the_vertical_sizing.unwrap_or_else(|| Sizing::RemainingSpace {});
        let horizontal_sizing = self.the_horizontal_sizing.unwrap_or_else(|| Sizing::RemainingSpace {});
        let horizontal_alignment = self.the_horizontal_alignment.unwrap_or_else(|| Alignment::Center);
        let vertical_alignment = self.the_vertical_alignment.unwrap_or_else(|| Alignment::Center);

        /* compute the model */
        let mut model = Model2D::new(vec!(), vec!(), true);
        let origin = compute_origin(&vertical_sizing, &horizontal_sizing, &vertical_alignment, &horizontal_alignment, &client);
        let antipode = compute_antipode(&vertical_sizing, &horizontal_sizing, &vertical_alignment, &horizontal_alignment, &client);
        panel.reassemble(&mut model, &origin, &antipode);

        Some(View {
            panel,
            model,
            vertical_sizing,
            horizontal_sizing,
            horizontal_alignment,
            vertical_alignment,
        })
    }
}
