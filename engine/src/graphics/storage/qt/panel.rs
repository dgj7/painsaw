use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::graphics::color::Color;
use crate::graphics::storage::m2d::Model2DBuilder;
use crate::graphics::storage::qt::layout::Layout;
use crate::graphics::storage::qt::sizing::Sizing;
use crate::graphics::storage::qt::widget::Widget;

///
/// a panel is a container for other [Panel]s and [Control]s.
///
pub struct Panel {
    /* nested panels and widgets */
    panels: Vec<Panel>,
    widgets: Vec<Widget>,

    /* representation of the requested size by the user for this element */
    vertical_sizing: Sizing,
    horizontal_sizing: Sizing,

    /* how nested elements fit within this panel */
    padding: f32,                   // todo: change padding to use Sizing
    layout: Layout,
}

impl Panel {
    ///
    /// reassemble the panel's model via model builder.
    ///
    pub fn reassemble(&self, builder: &mut Model2DBuilder, origin: &Vertex2D, antipode: &Vertex2D) {
        std::mem::take(builder)
            .with_primitive(Primitive2DBuilder::new()
                .with_mode(PolygonMode::Line)
                .with_color(Color::YELLOW)
                .with_type(PrimitiveType::Cube {})
                .with_vertex(origin.clone())
                .with_vertex(Vertex2D::new(antipode.x, origin.y))
                .with_vertex(antipode.clone())
                .with_vertex(Vertex2D::new(origin.x, antipode.y))
                .build());
        // todo -----v
        //self.panels.iter().for_each(|s| s.reassemble(builder));
        //self.widgets.iter().for_each(|w| w.reassemble(builder));
    }

    ///
    /// handle a click if it's vertex is within this panel's area.
    ///
    pub fn handle_click(&self, location: &Vertex2D) {
        // todo: make this more efficient
        self.panels.iter().for_each(|p| p.handle_click(location));
        self.widgets.iter().for_each(|w| w.handle_click(location));
    }
}

///
/// fluent builder for easier creation of panels.
///
pub struct PanelBuilder {
    /* nested panels and widgets */
    the_panels: Vec<Panel>,
    the_widgets: Vec<Widget>,

    /* how nested elements fit within this panel */
    the_padding: Option<f32>,
    the_layout: Option<Layout>,

    /* how this panel fits within it's parent area */
    the_vertical_sizing: Option<Sizing>,
    the_horizontal_sizing: Option<Sizing>,
}

impl PanelBuilder {
    pub fn new() -> Self {
        PanelBuilder {
            the_panels: vec!(),
            the_widgets: vec!(),

            the_padding: None,
            the_layout: None,

            the_vertical_sizing: None,
            the_horizontal_sizing: None,
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

    pub fn with_vertical_sizing(mut self, sizing: Sizing) -> Self {
        self.the_vertical_sizing = Some(sizing);
        self
    }

    pub fn with_horizontal_sizing(mut self, sizing: Sizing) -> Self {
        self.the_horizontal_sizing = Some(sizing);
        self
    }

    pub fn build(self) -> Option<Panel> {
        if self.the_horizontal_sizing.is_none() || self.the_vertical_sizing.is_none() {
            return None;
        }

        let vertical_sizing = self.the_horizontal_sizing.unwrap();
        let horizontal_sizing = self.the_vertical_sizing.unwrap();

        let panel = Panel {
            vertical_sizing,
            horizontal_sizing,

            panels: self.the_panels,
            widgets: self.the_widgets,

            padding: self.the_padding.unwrap_or_else(|| 5.0),
            layout: self.the_layout.unwrap_or_else(|| Layout::Horizontal),
        };

        Some(panel)
    }
}
