use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::graphics::color::Color;
use crate::graphics::storage::m2d::{Model2D, Model2DBuilder};
use crate::graphics::storage::qt::attrib::sizing::Sizing;

///
/// a widget is any control that can be clicked on screen.
///
pub struct Widget {
    /* representation of the requested size by the user for this element */
    vertical_sizing: Sizing,
    horizontal_sizing: Sizing,

    /* how to handle when a click has registered */
    click_action: fn(pt: &Vertex2D),
}

impl Widget {
    ///
    /// handle click.
    ///
    pub fn handle_click(&self, pt: &Vertex2D) {
        (self.click_action)(pt);
    }
    
    ///
    /// reassemble the control.
    ///
    pub fn reassemble(&self, model: &mut Model2D, origin: &Vertex2D, antipode: &Vertex2D) {
        model.primitives
            .push(Primitive2DBuilder::new()
            .with_mode(PolygonMode::Fill)
            .with_color(Color::RED)
            .with_type(PrimitiveType::Cube {})
            .with_vertex(origin.clone())
            .with_vertex(Vertex2D::new(antipode.x, origin.y))
            .with_vertex(antipode.clone())
            .with_vertex(Vertex2D::new(origin.x, antipode.y))
            .build());
    }
}

///
/// fluent builder for easier creation of widgets.
/// 
pub struct WidgetBuilder {
    the_vertical_sizing: Option<Sizing>,
    the_horizontal_sizing: Option<Sizing>,
    the_click_action: Option<fn(pt: &Vertex2D)>,
}

impl WidgetBuilder {
    pub fn new() -> Self {
        WidgetBuilder {
            the_vertical_sizing: None,
            the_horizontal_sizing: None,
            the_click_action: None,
        }
    }

    pub fn with_vertical_sizing(mut self, sizing: Sizing) -> Self {
        self.the_vertical_sizing = Some(sizing);
        self
    }

    pub fn with_horizontal_sizing(mut self, sizing: Sizing) -> Self {
        self.the_horizontal_sizing = Some(sizing);
        self
    }

    pub fn with_click_action(mut self, click_action: fn(pt: &Vertex2D)) -> Self {
        self.the_click_action = Some(click_action);
        self
    }

    pub fn build(self) -> Option<Widget> {
        if self.the_horizontal_sizing.is_none() || self.the_vertical_sizing.is_none() {
            return None;
        }

        let vertical_sizing = self.the_horizontal_sizing.unwrap();
        let horizontal_sizing = self.the_vertical_sizing.unwrap();

        let widget = Widget {
            vertical_sizing,
            horizontal_sizing,

            click_action: self.the_click_action.unwrap_or_else(|| |_vertex|{}),
        };

        Some(widget)
    }
}
