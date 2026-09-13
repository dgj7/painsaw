use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::PrimitiveType;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::color::Color;
use crate::graphics::storage::m2d::Model2DBuilder;

///
/// a widget is any control that can be clicked on screen.
///
pub struct Widget {
    origin: Vertex2D,
    size: Dimension2D,

    click_action: fn(pt: &Vertex2D),
    
    redraw_necessary: bool,
}

impl Widget {
    ///
    /// create a new instance.
    /// 
    pub fn new(origin: Vertex2D, size: Dimension2D, click_action: fn(pt: &Vertex2D)) -> Self {
        Widget {
            origin,
            size,
            click_action,
            redraw_necessary: true,
        }
    }

    ///
    /// handle click.
    ///
    pub fn handle_click(&self, pt: &Vertex2D) {
        (self.click_action)(pt);
    }
    
    ///
    /// reassemble the control.
    ///
    pub fn reassemble(&self, builder: &mut Model2DBuilder) {
        if !self.redraw_necessary {
            return;
        }
        
        std::mem::take(builder)
            .with_primitive(Primitive2DBuilder::new()
                .with_mode(PolygonMode::Fill)
                .with_color(Color::RED)
                .with_type(PrimitiveType::Cube {})
                .with_vertex(self.origin.clone())
                .with_vertex(Vertex2D::new(self.origin.x + self.size.width, self.origin.y))
                .with_vertex(Vertex2D::new(self.origin.x + self.size.width, self.origin.y + self.size.height))
                .with_vertex(Vertex2D::new(self.origin.x, self.origin.y + self.size.height))
                .build());
    }
}

///
/// fluent builder for easier creation of widgets.
/// 
pub struct WidgetBuilder {
    the_origin: Option<Vertex2D>,
    the_size: Option<Dimension2D>,
    the_click_action: Option<fn(pt: &Vertex2D)>,
}

impl WidgetBuilder {
    pub fn new() -> Self {
        WidgetBuilder {
            the_origin: None,
            the_size: None,
            the_click_action: None,
        }
    }

    pub fn with_origin(mut self, origin: Vertex2D) -> Self {
        self.the_origin = Some(origin);
        self
    }

    pub fn with_size(mut self, size: Dimension2D) -> Self {
        self.the_size = Some(size);
        self
    }

    pub fn with_click_action(mut self, click_action: fn(pt: &Vertex2D)) -> Self {
        self.the_click_action = Some(click_action);
        self
    }

    pub fn build(self) -> Widget {
        Widget {
            origin: self.the_origin.unwrap_or_else(|| Vertex2D::new(100.0, 100.0)),
            size: self.the_size.unwrap_or_else(|| Dimension2D::new(100.0, 100.0)),
            click_action: self.the_click_action.unwrap_or_else(|| |vertex|{}),
            redraw_necessary: true,
        }
    }
}
