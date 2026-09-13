use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};
use crate::graphics::storage::qt::panel::Panel;
use crate::graphics::storage::qt::sizing::Sizing;
use crate::input::screen::ScreenState;

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
        let mut builder = Model2DBuilder::new();
        let antipode = compute_antipode(&self.vertical, &self.horizontal, screen);
        
        self.panel.reassemble(&mut builder, &self.origin, &antipode);
        
        self.model = builder.build();
    }
}

fn compute_antipode(vertical: &Sizing, horizontal: &Sizing, screen: &ScreenState) -> Vertex2D {
    let x = horizontal.screen_dimension_to_actual(screen.current_client_dimensions.width);
    let y = vertical.screen_dimension_to_actual(screen.current_client_dimensions.height);
    Vertex2D { x, y }
}
