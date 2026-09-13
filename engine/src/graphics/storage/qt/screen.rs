use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};
use crate::graphics::storage::qt::panel::Panel;

pub struct Screen {
    pub panel: Panel,
    pub origin: Vertex2D,
    pub antipode: Vertex2D,
    pub model: Model2D,
}

impl Screen {
    pub fn resize(&mut self) {
        let mut builder = Model2DBuilder::new();
        self.panel.reassemble(&mut builder, &self.origin, &self.antipode);
        self.model = builder.build();
    }
}
