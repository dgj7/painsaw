use crate::geometry::primitive::face::PolygonFace;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::geometry::rect::Rectangle2D;
use crate::graphics::color::Color;
use crate::graphics::storage::g2d::m2d::Model2D;
use crate::graphics::storage::ui::view::attrib::assembled::Assembled;
use crate::graphics::storage::ui::view::attrib::layout::Layout;
use crate::graphics::storage::ui::view::attrib::sizing::Sizing;
use crate::graphics::storage::ui::view::widget::Widget;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use std::collections::HashMap;

///
/// a panel is a container for other [Panel]s and [Control]s.
///
pub struct Panel {
    /* nested panels and widgets */
    panels: HashMap<u32, (Panel, Sizing)>,
    widgets: HashMap<u32, (Widget, Sizing)>,

    /* store elements added to the panel; we need both of these so that we can support removal and addition of elements at runtime */
    order: Vec<u32>,

    /* how nested elements are rendered onto this panel */
    layout: Layout,
}

impl Panel {
    ///
    /// handle a click if it's vertex is within this panel's area.
    ///
    pub fn click(&self, location: &Vertex2D) {
        // todo: make this more efficient
        self.panels.iter().for_each(|(_key, (panel, _sizing))| panel.click(location));
        self.widgets.iter().for_each(|(_key, (widget, _sizing))| widget.handle_click(location));
    }

    ///
    /// get the element with the given id.
    ///
    fn element_at(&self, index: u32) -> Option<(Option<&(Panel, Sizing)>, Option<&(Widget, Sizing)>)> {
        if self.panels.contains_key(&index) {
            Some((self.panels.get(&index), None))
        } else if self.widgets.contains_key(&index) {
            Some((None, self.widgets.get(&index)))
        } else {
            None
        }
    }
}

impl Assembled for Panel {
    fn reassemble(&self, debug: bool, model: &mut Model2D, rectangle: &Rectangle2D) {
        if debug {
            model.primitives.push(Primitive2DBuilder::new()
                .with_mode(PolygonMode::Line)
                .with_face(PolygonFace::FrontAndBack)
                .with_color(Color::YELLOW)
                .with_type(PrimitiveType::Cube { thickness: 1.0 })
                .with_vertex(rectangle.origin.clone())
                .with_vertex(Vertex2D::new(rectangle.antipode.x, rectangle.origin.y))
                .with_vertex(rectangle.antipode.clone())
                .with_vertex(Vertex2D::new(rectangle.origin.x, rectangle.antipode.y))
                .build());
            model.primitives.push(Primitive2DBuilder::new()
                .with_color(Color::GREEN)
                .with_type(PrimitiveType::Point { point_size: 3.0 })
                .with_vertex(rectangle.origin.clone())
                .build());
        }

        log(LogLevel::Info, &|| format!("panel assembled: origin=({},{}),antipode=({},{})", rectangle.origin.x, rectangle.origin.y, rectangle.antipode.x, rectangle.antipode.y));

        let mut remaining = rectangle.clone();
        for c in self.order.iter() {
            log(LogLevel::Info, &|| format!("remaining: o=({},{}),a=({},{})", remaining.origin.x, remaining.origin.y, remaining.antipode.x, remaining.antipode.y));
            if !reassemble_element(debug, &self, *c, &self.layout, rectangle, &mut remaining, model) {
                break
            }
        }
    }
}

fn reassemble_element(
    debug: bool,
    panel: &Panel,
    c: u32,
    layout: &Layout,
    rectangle: &Rectangle2D,
    remaining: &mut Rectangle2D,
    model: &mut Model2D,
) -> bool {
    if let Some(element) = panel.element_at(c) {
        return if let Some((panel, sizing)) = element.0 {
            reassemble_concrete_element(debug, panel, layout, rectangle, remaining, sizing, model)
        } else if let Some((widget, sizing)) = element.1 {
            reassemble_concrete_element(debug, widget, layout, rectangle, remaining, sizing, model)
        } else {
            false
        }
    }
    false
}

fn reassemble_concrete_element<T: Assembled>(debug: bool, assembled: &T, layout: &Layout, rectangle: &Rectangle2D, remaining: &mut Rectangle2D, sizing: &Sizing, model: &mut Model2D) -> bool {
    let next = layout.determine_next(rectangle, &remaining, sizing);
    if rectangle.contains_rect_inclusive(&next) {
        layout.subtract(remaining, &next);
        assembled.reassemble(debug, model, &next);
        true
    } else {
        log(LogLevel::Warning, &|| format!("next (({},{}),({},{})) doesn't fit in (({},{}),({},{}))", next.origin.x, next.origin.y, next.antipode.x, next.antipode.y, rectangle.origin.x, rectangle.origin.y, rectangle.antipode.x, rectangle.antipode.y));
        false
    }
}

///
/// fluent builder for easier creation of panels.
///
pub struct PanelBuilder {
    /* nested panels and widgets */
    the_panels: HashMap<u32, (Panel, Sizing)>,
    the_widgets: HashMap<u32, (Widget, Sizing)>,

    /* elements added */
    the_count: u32,
    the_order: Vec<u32>,

    /* how nested elements fit within this panel */
    the_layout: Option<Layout>,
}

impl PanelBuilder {
    pub fn new() -> Self {
        PanelBuilder {
            the_panels: HashMap::new(),
            the_widgets: HashMap::new(),

            the_count: 0,
            the_order: Vec::new(),

            the_layout: None,
        }
    }

    pub fn with_panel(mut self, panel: Panel, sizing: Sizing) -> Self {
        self.the_count = self.the_count + 1;
        self.the_panels.insert(self.the_count, (panel, sizing));
        self.the_order.push(self.the_count);
        self
    }

    pub fn with_widget(mut self, widget: Widget, sizing: Sizing) -> Self {
        self.the_count = self.the_count + 1;
        self.the_widgets.insert(self.the_count, (widget, sizing));
        self.the_order.push(self.the_count);
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.the_layout = Some(layout);
        self
    }

    pub fn build(self) -> Option<Panel> {
        if self.the_layout.is_none() {
            return None;
        }

        let panel = Panel {
            order: self.the_order,

            panels: self.the_panels,
            widgets: self.the_widgets,

            layout: self.the_layout.unwrap(),
        };

        Some(panel)
    }
}
