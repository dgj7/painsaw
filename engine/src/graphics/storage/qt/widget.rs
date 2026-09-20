use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::geometry::rect::Rectangle2D;
use crate::graphics::color::Color;
use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::assembled::Assembled;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

///
/// a widget is any control that can be clicked on screen.
///
pub struct Widget {
    click_action: fn(pt: &Vertex2D),    /* how to handle when a click has registered */

    padding: f32,
}

impl Widget {
    ///
    /// handle click.
    ///
    pub fn handle_click(&self, pt: &Vertex2D) {
        (self.click_action)(pt);
    }
}

impl Assembled for Widget {
    fn reassemble(&self, model: &mut Model2D, rectangle: &Rectangle2D) {
        model.primitives
            .push(Primitive2DBuilder::new()
                .with_mode(PolygonMode::Fill)
                .with_color(Color::RED)
                .with_type(PrimitiveType::Cube { thickness: 1.0 })
                .with_vertex(Vertex2D::new(rectangle.origin.x + self.padding, rectangle.origin.y + self.padding))         // top left (origin)
                .with_vertex(Vertex2D::new(rectangle.antipode.x - self.padding, rectangle.origin.y + self.padding))       // top right
                .with_vertex(Vertex2D::new(rectangle.antipode.x - self.padding, rectangle.antipode.y - self.padding))     // bottom right (antipode)
                .with_vertex(Vertex2D::new(rectangle.origin.x + self.padding, rectangle.antipode.y-self.padding))       // bottom left
                .build());
        log(LogLevel::Info, &|| format!("widget assembled: origin=({},{}),antipode=({},{})", rectangle.origin.x, rectangle.origin.y, rectangle.antipode.x, rectangle.antipode.y));
    }
}

///
/// fluent builder for easier creation of widgets.
/// 
pub struct WidgetBuilder {
    the_click_action: Option<fn(pt: &Vertex2D)>,

    the_padding: Option<f32>,
}

impl WidgetBuilder {
    pub fn new() -> Self {
        WidgetBuilder {
            the_click_action: None,

            the_padding: None,
        }
    }

    pub fn with_click_action(mut self, click_action: fn(pt: &Vertex2D)) -> Self {
        self.the_click_action = Some(click_action);
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.the_padding = Some(padding);
        self
    }

    pub fn build(self) -> Widget {
        Widget {
            click_action: self.the_click_action.unwrap_or_else(|| |_vertex|{}),

            padding: self.the_padding.unwrap_or_else(|| 10.0),
        }
    }
}
