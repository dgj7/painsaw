use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::PrimitiveType;
use crate::geometry::primitive::v2d::Vertex2D;
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

    /* how nested elements fit within this panel */
    padding: f32,
    layout: Layout,
    next: Vertex2D,

    /* how this panel fits within it's parent area */
    origin: Vertex2D,
    external_vertical_sizing: Sizing,
    external_horizontal_sizing: Sizing,
    height: f32,
    width: f32,

    /* whether this panel (NOT it's nested panels or widgets) needs to be redrawn */
    redraw_necessary: bool,
}

impl Panel {
    ///
    /// reassemble the panel.
    ///
    pub fn reassemble(&self, builder: &mut Model2DBuilder) {
        std::mem::take(builder)
            .with_primitive(Primitive2DBuilder::new()
                .with_mode(PolygonMode::Line)
                .with_color(Color::YELLOW)
                .with_type(PrimitiveType::Cube {})
                .with_vertex(self.origin.clone())
                .with_vertex(Vertex2D::new(self.origin.x + self.width, self.origin.y))
                .with_vertex(Vertex2D::new(self.origin.x + self.width, self.origin.y + self.height))
                .with_vertex(Vertex2D::new(self.origin.x, self.origin.y + self.height))
                .build());
        self.panels.iter().for_each(|s| s.reassemble(builder));
        self.widgets.iter().for_each(|w| w.reassemble(builder));
    }

    ///
    /// determine if we need to redraw.
    ///
    pub fn is_redraw_necessary(&self) -> bool {
        let redraw_panels = self.panels
            .iter()
            .filter(|p| p.is_redraw_necessary())
            .map(|p| p.is_redraw_necessary())
            .next()
            .unwrap_or(false);
        let redraw_widgets = self.widgets
            .iter()
            .filter(|w| w.redraw_necessary)
            .map(|w| w.redraw_necessary)
            .next()
            .unwrap_or(false);
        self.redraw_necessary || redraw_panels || redraw_widgets
    }

    ///
    /// handle a click if it's vertex is within this panel's area.
    ///
    pub fn handle_click(&self, location: &Vertex2D) {
        // todo: make this more efficient
        self.panels.iter().for_each(|p| p.handle_click(location));
        self.widgets.iter().for_each(|w| w.handle_click(location));
    }

    ///
    /// resize the panel.
    ///
    pub fn resize(&mut self, dim: &Dimension2D) {
        self.height = dim.height;
        self.width = dim.width;
        self.redraw_necessary = true;
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
    the_origin: Option<Vertex2D>,
    the_external_vertical_sizing: Option<Sizing>,
    the_external_horizontal_sizing: Option<Sizing>,
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
            the_external_vertical_sizing: None,
            the_external_horizontal_sizing: None,
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

    pub fn with_vertical_sizing(mut self, sizing: Sizing) -> Self {
        self.the_external_vertical_sizing = Some(sizing);
        self
    }

    pub fn with_horizontal_sizing(mut self, sizing: Sizing) -> Self {
        self.the_external_horizontal_sizing = Some(sizing);
        self
    }

    pub fn with_size(mut self, size: Dimension2D) -> Self {
        self.the_size = Some(size);
        self
    }

    pub fn build(self) -> Option<Panel> {
        if self.the_origin.is_none() || self.the_size.is_none() {
            return None;
        }

        let origin = self.the_origin.unwrap();
        let size = self.the_size.unwrap();
        let external_vertical_sizing = self.the_external_horizontal_sizing.unwrap();
        let external_horizontal_sizing = self.the_external_vertical_sizing.unwrap();

        let panel = Panel {
            panels: self.the_panels,
            widgets: self.the_widgets,

            padding: self.the_padding.unwrap_or_else(|| 5.0),
            layout: self.the_layout.unwrap_or_else(|| Layout::Horizontal),
            next: origin.clone(),

            origin,
            external_vertical_sizing,
            external_horizontal_sizing,
            width: size.width,
            height: size.height,

            redraw_necessary: true,
        };

        Some(panel)
    }
}
