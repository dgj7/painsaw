use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::PrimitiveType;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::graphics::color::Color;
use crate::graphics::storage::m2d::Model2DBuilder;
use crate::graphics::storage::qt::layout::Layout;
use crate::graphics::storage::qt::widget::Widget;

///
/// a panel is a container for other [Panel]s and [Control]s.
///
pub struct Panel {
    panels: Vec<Panel>,
    widgets: Vec<Widget>,

    padding: f32,
    layout: Layout,

    origin: Vertex2D,
    size: Dimension2D,

    redraw_necessary: bool,
}

impl Panel {
    ///
    /// reassemble the panel.
    ///
    pub fn reassemble(&self, builder: &mut Model2DBuilder) {
        if !self.redraw_necessary {
            return;
        }

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
        self.panels.iter().for_each(|s| s.reassemble(builder));
        self.widgets.iter().for_each(|w| w.reassemble(builder));
    }

    ///
    /// handle a click if it's vertex is within this panel's area.
    ///
    pub fn handle_click(&self, location: &Vertex2D) {
        // todo
    }

    ///
    /// resize the panel.
    ///
    pub fn resize(&mut self, dim: &Dimension2D) {
        self.size = dim.clone();
        self.redraw_necessary = true;
    }
}

///
/// fluent builder for easier creation of panels.
///
pub struct PanelBuilder {
    the_panels: Vec<Panel>,
    the_widgets: Vec<Widget>,

    the_padding: Option<f32>,
    the_layout: Option<Layout>,

    the_origin: Option<Vertex2D>,
    the_size: Option<Dimension2D>,
}

impl PanelBuilder {
    pub fn new() -> Self {
        PanelBuilder {
            the_panels: vec!(),
            the_widgets: vec!(),

            the_padding: None,
            the_layout: None,

            the_origin: None,
            the_size: None,
        }
    }

    pub fn with_panel(mut self, panel: Panel) -> Self {
        self.the_panels.push(panel);
        self
    }

    pub fn with_widget(mut self, widget: Widget) -> Self {
        self.the_widgets.push(widget);
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.the_padding = Some(padding);
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.the_layout = Some(layout);
        self
    }

    pub fn with_origin(mut self, origin: Vertex2D) -> Self {
        self.the_origin = Some(origin);
        self
    }

    pub fn with_size(mut self, size: Dimension2D) -> Self {
        self.the_size = Some(size);
        self
    }

    pub fn build(self) -> Panel {
        Panel {
            panels: self.the_panels,
            widgets: self.the_widgets,

            padding: self.the_padding.unwrap_or_else(|| 5.0),
            layout: self.the_layout.unwrap_or_else(|| Layout::Horizontal),

            origin: self.the_origin.unwrap_or_else(|| Vertex2D::new(100.0, 100.0)),
            size: self.the_size.unwrap_or_else(|| Dimension2D::new(100.0, 100.0)),

            redraw_necessary: true,
        }
    }
}
