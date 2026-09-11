use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::storage::m2d::Model2DBuilder;
use crate::graphics::storage::qt::control::Control;
use crate::graphics::storage::qt::sizer::Sizer;
use crate::graphics::storage::qt::support::clickable::Clickable;
use crate::graphics::storage::qt::support::dimensional::Dimensional;
use crate::graphics::storage::qt::support::padded::Padded;

pub struct HorizontalSizer {
    pub sizers: Vec<Box<dyn Sizer>>,
    pub controls: Vec<Box<dyn Control>>,

    click_action: fn(pt: &Vertex2D),

    pub padding: f32,

    origin: Vertex2D,
    size: Dimension2D,
}

impl Clickable for HorizontalSizer {
    fn click(&mut self, point: &Vertex2D) {
        (self.click_action)(point);
    }
}

impl Padded for HorizontalSizer { fn padding(&self) -> f32 { self.padding } }

impl Sizer for HorizontalSizer {
    fn add_sizer(&mut self, _sizer: Box<dyn Sizer>) {
        todo!()
    }

    fn add_control(&mut self, _control: Box<dyn Control>) {
        todo!()
    }

    fn resize(&mut self, dim: &Dimension2D) {
        self.size = dim.clone();
    }

    fn assemble(&self, _builder: &mut Model2DBuilder) {
        todo!()
    }
}

impl Dimensional for HorizontalSizer {
    fn height(&self) -> f32 {
        self.size.height - self.origin.y
    }

    fn width(&self) -> f32 {
        self.size.width - self.origin.x
    }
}
