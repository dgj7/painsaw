use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::graphics::color::Color;
use crate::graphics::storage::m2d::Model2DBuilder;
use crate::graphics::storage::qt::control::Control;
use crate::graphics::storage::qt::sizer::Sizer;
use crate::graphics::storage::qt::support::assembled::Assembled;
use crate::graphics::storage::qt::support::clickable::Clickable;
use crate::graphics::storage::qt::support::dimensional::Dimensional;
use crate::graphics::storage::qt::support::padded::Padded;
use crate::graphics::storage::qt::support::resizable::Resizable;

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
    fn add_sizer(&mut self, sizer: Box<dyn Sizer>) {
        self.sizers.push(sizer);
    }

    fn add_control(&mut self, control: Box<dyn Control>) {
        self.controls.push(control);
    }
}

impl Resizable for HorizontalSizer {
    fn resize(&mut self, dim: &Dimension2D) {
        self.size = dim.clone();
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

impl Assembled for HorizontalSizer {
    fn reassemble(&self, builder: &mut Model2DBuilder) {
        std::mem::take(builder)
            .with_primitive(Primitive2DBuilder::new()
                .with_mode(PolygonMode::Line)
                .with_color(Color::YELLOW)
                .with_type(PrimitiveType::Cube {})
                .with_vertex(self.origin.clone())
                .with_vertex(Vertex2D::new(self.origin.x + self.size.width, self.origin.y))
                .with_vertex(Vertex2D::new(self.origin.x + self.size.width, self.origin.y + self.size.height))
                .with_vertex(Vertex2D::new(self.origin.x, self.origin.y + self.size.height))
                .build());
    }
}
