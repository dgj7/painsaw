use crate::geometry::primitive::face::PolygonFace;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::graphics::color::Color;
use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::attrib::layout::Layout;
use crate::graphics::storage::qt::attrib::sizing::Sizing;
use crate::graphics::storage::qt::sr::SizingRequest;
use crate::graphics::storage::qt::widget::Widget;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use std::collections::HashMap;

///
/// a panel is a container for other [Panel]s and [Control]s.
///
pub struct Panel {
    /* nested panels and widgets */
    panels: HashMap<u32, Panel>,
    widgets: HashMap<u32, Widget>,

    /* store elements added to the panel; we need both of these so that we can support removal and addition of elements at runtime */
    count: u32,
    order: Vec<u32>,

    /* requested size by the user for this panel */
    sizing: SizingRequest,

    /* how nested elements fit within this panel */
    padding: Sizing,
    layout: Layout,
}

impl Panel {
    ///
    /// reassemble the panel's model via model builder.
    ///
    pub fn reassemble(&self, model: &mut Model2D, origin: &Vertex2D, antipode: &Vertex2D) {
        log(LogLevel::Debug, &|| format!("reassembling panel: ({},{}),({},{}),({},{}),({},{})", origin.x, origin.y, antipode.x, origin.y, antipode.x, antipode.y, origin.x, antipode.y));
        let pb = Primitive2DBuilder::new()
            .with_mode(PolygonMode::Line)
            .with_face(PolygonFace::FrontAndBack)
            .with_color(Color::YELLOW)
            .with_type(PrimitiveType::Cube { thickness: 3.0 })
            .with_vertex(origin.clone())
            .with_vertex(Vertex2D::new(antipode.x, origin.y))
            .with_vertex(antipode.clone())
            .with_vertex(Vertex2D::new(origin.x, antipode.y));
        model.primitives.push(pb.build());

        for c in self.order.iter() {
            if let Some(element) = self.element_at(*c) {
                if let Some(panel) = element.0 {
                    panel.reassemble(model, origin, antipode);
                } else if let Some(widget) = element.1 {
                    widget.reassemble(model, origin, antipode);
                }
            }
        }

        log(LogLevel::Debug, &|| String::from("done reassembling panel"));
    }

    ///
    /// handle a click if it's vertex is within this panel's area.
    ///
    pub fn handle_click(&self, location: &Vertex2D) {
        // todo: make this more efficient
        self.panels.iter().for_each(|(key, panel)| panel.handle_click(location));
        self.widgets.iter().for_each(|(key, widget)| widget.handle_click(location));
    }

    fn element_at(&self, index: u32) -> Option<(Option<&Panel>, Option<&Widget>)> {
        if self.panels.contains_key(&index) {
            Some((self.panels.get(&index), None))
        } else if self.widgets.contains_key(&index) {
            Some((None, self.widgets.get(&index)))
        } else {
            None
        }
    }
}

///
/// fluent builder for easier creation of panels.
///
pub struct PanelBuilder {
    /* nested panels and widgets */
    the_panels: HashMap<u32, Panel>,
    the_widgets: HashMap<u32, Widget>,

    /* elements added */
    the_count: u32,
    the_order: Vec<u32>,

    /* how nested elements fit within this panel */
    the_padding: Option<Sizing>,
    the_layout: Option<Layout>,

    /* how this panel fits within it's parent area */
    the_sizing: Option<SizingRequest>,
}

impl PanelBuilder {
    pub fn new() -> Self {
        PanelBuilder {
            the_panels: HashMap::new(),
            the_widgets: HashMap::new(),

            the_count: 0,
            the_order: Vec::new(),

            the_padding: None,
            the_layout: None,

            the_sizing: None,
        }
    }

    pub fn with_panel(mut self, panel: Panel) -> Self {
        self.the_count = self.the_count + 1;
        self.the_panels.insert(self.the_count, panel);
        self.the_order.push(self.the_count);
        self
    }

    pub fn with_widget(mut self, widget: Widget) -> Self {
        self.the_count = self.the_count + 1;
        self.the_widgets.insert(self.the_count, widget);
        self.the_order.push(self.the_count);
        self
    }

    pub fn with_padding(mut self, padding: Sizing) -> Self {
        self.the_padding = Some(padding);
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.the_layout = Some(layout);
        self
    }

    pub fn with_sizing(mut self, sizing: SizingRequest) -> Self {
        self.the_sizing = Some(sizing);
        self
    }

    pub fn build(self) -> Option<Panel> {
        let panel = Panel {
            count: self.the_count,
            order: self.the_order,
            panels: self.the_panels,
            widgets: self.the_widgets,

            padding: self.the_padding.unwrap_or_else(|| Sizing::Exact { size: 5.0 }),
            layout: self.the_layout.unwrap_or_else(|| Layout::Horizontal),

            sizing: self.the_sizing.unwrap_or_else(|| SizingRequest::default()),
        };

        Some(panel)
    }
}
