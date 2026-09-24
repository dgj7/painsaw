use std::collections::HashMap;
use crate::geometry::dim::Dimension2D;
use crate::geometry::primitive::face::PolygonFace;
use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::geometry::rect::Rectangle2D;
use crate::graphics::color::Color;
use crate::graphics::storage::g2d::m2d::Model2D;
use crate::graphics::storage::ui::view::qt::QuadTree;
use crate::input::screen::ScreenState;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use attrib::align::Alignment;
use attrib::assembled::Assembled;
use attrib::sizing::Sizing;
use panel::Panel;
use crate::support::id::Identifier;

pub mod attrib;
pub mod panel;
pub mod widget;
pub(crate) mod qt;

pub struct View {
    pub panel: Panel,                       /* the main panel */
    pub model: Model2D,                     /* a renderable representation of the screen and it's main panel */

    pub vertical_sizing: Sizing,            /* size of the view on screen, in both directions */
    pub horizontal_sizing: Sizing,

    pub vertical_alignment: Alignment,      /* alignment of the view on screen, in both directions */
    pub horizontal_alignment: Alignment,

    /* storage for rectangle collisions */
    qt: QuadTree,
    clicks: HashMap<Identifier, fn(pt: &Vertex2D)>,

    /* optional rendering choices */
    debug_enabled: bool,
    background: Option<Color>,
    border: Option<(Color, f32)>,
}

impl View {
    pub fn resize(&mut self, screen: &ScreenState) {
        /* create a new model, and determine the view rectangle */
        let mut model = Model2D::new(vec!(), vec!(), true);
        let rectangle = client_to_sized_rectangle(&screen.current_client_dimensions, &self.vertical_sizing, &self.horizontal_sizing, &self.vertical_alignment, &self.horizontal_alignment);

        /* re-initialize quadtree */
        self.qt = QuadTree::new(rectangle.clone());

        /* render the background, if requested */
        if let Some(bg) = self.background {
            model.primitives.push(Primitive2DBuilder::new()
                .with_type(PrimitiveType::Cube { thickness: 1.0 })
                .with_face(PolygonFace::Front)
                .with_mode(PolygonMode::Fill)
                .with_color(bg)
                .with_vertices(rectangle.to_vertices())
                .build());
        }

        /* render the border, if requested */
        if let Some((color, thickness)) = self.border {
            model.primitives.push(Primitive2DBuilder::new()
                .with_type(PrimitiveType::Cube { thickness })
                .with_mode(PolygonMode::Line)
                .with_color(color)
                .with_vertices(rectangle.to_vertices())
                .build());
        }

        /* reassemble the model */
        self.panel.reassemble(self.debug_enabled, &mut model, &rectangle, &mut self.qt);
        self.model = model;
    }

    ///
    /// handle a click at the given location.
    ///
    pub fn click(&self, location: &Vertex2D) {
        let ids = self.qt.query(location);
        for id in ids {
            if let Some(click) = self.clicks.get(&id) {
                (click)(location);
            }
        }
    }

    ///
    /// load click handlers into a map.
    ///
    pub(super) fn load_click_handlers(&mut self) {
        self.panel.load_click_handlers(&mut self.clicks);
    }
}

pub struct ViewBuilder {
    the_panel: Option<Panel>,

    the_vertical_sizing: Option<Sizing>,
    the_horizontal_sizing: Option<Sizing>,

    the_vertical_alignment: Option<Alignment>,
    the_horizontal_alignment: Option<Alignment>,

    the_debug_enabled: Option<bool>,
    the_background: Option<Color>,
    the_border: Option<(Color, f32)>,

    the_window_dimensions: Option<Dimension2D>,
}

impl ViewBuilder {
    pub fn new() -> ViewBuilder {
        ViewBuilder {
            the_panel: None,

            the_vertical_sizing: None,
            the_horizontal_sizing: None,

            the_vertical_alignment: None,
            the_horizontal_alignment: None,

            the_debug_enabled: None,
            the_background: None,
            the_border: None,

            the_window_dimensions: None,
        }
    }

    pub fn with_panel(mut self, panel: Panel) -> ViewBuilder {
        self.the_panel = Some(panel);
        self
    }

    pub fn with_vertical_sizing(mut self, sizing: Sizing) -> ViewBuilder {
        self.the_vertical_sizing = Some(sizing);
        self
    }

    pub fn with_horizontal_sizing(mut self, sizing: Sizing) -> ViewBuilder {
        self.the_horizontal_sizing = Some(sizing);
        self
    }

    pub fn with_vertical_alignment(mut self, alignment: Alignment) -> ViewBuilder {
        self.the_vertical_alignment = Some(alignment);
        self
    }

    pub fn with_horizontal_alignment(mut self, alignment: Alignment) -> ViewBuilder {
        self.the_horizontal_alignment = Some(alignment);
        self
    }

    pub fn with_debug_enabled(mut self, debug: bool) -> ViewBuilder {
        self.the_debug_enabled = Some(debug);
        self
    }

    pub fn with_background(mut self, color: Color) -> ViewBuilder {
        self.the_background = Some(color);
        self
    }

    pub fn with_border(mut self, color: Color, thickness: f32) -> ViewBuilder {
        self.the_border = Some((color, thickness));
        self
    }
    
    pub fn with_window_dimensions(mut self, dimensions: Dimension2D) -> ViewBuilder {
        self.the_window_dimensions = Some(dimensions);
        self
    }

    pub fn build(self) -> Option<View> {
        /* none if required fields are missing */
        if self.the_panel.is_none() || self.the_window_dimensions.is_none() {
            log(LogLevel::Warning, &|| String::from("didn't provide panel or client"));
            return None;
        }

        /* required fields */
        let panel = self.the_panel.unwrap();
        let window = self.the_window_dimensions.unwrap();

        /* pull optional fields, with replacements */
        let vertical_sizing = self.the_vertical_sizing.unwrap_or_else(|| Sizing::RemainingSpace {});
        let horizontal_sizing = self.the_horizontal_sizing.unwrap_or_else(|| Sizing::RemainingSpace {});
        let vertical_alignment = self.the_vertical_alignment.unwrap_or_else(|| Alignment::Center);
        let horizontal_alignment = self.the_horizontal_alignment.unwrap_or_else(|| Alignment::Center);

        /* compute the model */
        let mut model = Model2D::new(vec!(), vec!(), true);
        let rectangle = client_to_sized_rectangle(&window, &vertical_sizing, &horizontal_sizing, &vertical_alignment, &horizontal_alignment);
        let debug = self.the_debug_enabled.unwrap_or_else(|| false);
        let mut qt = QuadTree::new(rectangle.clone());
        panel.reassemble(debug, &mut model, &rectangle, &mut qt);

        /* now we need to assign click handlers */
        let mut view = View {
            panel,
            model,

            vertical_sizing,
            horizontal_sizing,

            vertical_alignment,
            horizontal_alignment,

            qt,
            clicks: HashMap::new(),

            debug_enabled: debug,
            background: self.the_background,
            border: self.the_border,
        };
        view.load_click_handlers();

        /* done */
        Some(view)
    }
}

fn client_to_origin(window: &Dimension2D, vertical_sizing: &Sizing, horizontal_sizing: &Sizing, vertical_alignment: &Alignment, horizontal_alignment: &Alignment) -> Vertex2D {
    let width = horizontal_sizing.container_to_dimension(window.width);
    let height = vertical_sizing.container_to_dimension(window.height);
    let x = match horizontal_alignment {
        Alignment::Minimum => 0.0,
        Alignment::Center => (window.width / 2.0) - (width / 2.0),
        Alignment::Maximum => window.width - width,
    };
    let y = match vertical_alignment {
        Alignment::Minimum => 0.0,
        Alignment::Center => (window.height / 2.0) - (height / 2.0),
        Alignment::Maximum => window.height - height,
    };
    Vertex2D { x, y }
}

fn client_to_antipode(window: &Dimension2D, vertical_sizing: &Sizing, horizontal_sizing: &Sizing, vertical_alignment: &Alignment, horizontal_alignment: &Alignment) -> Vertex2D {
    let width = horizontal_sizing.container_to_dimension(window.width);
    let height = vertical_sizing.container_to_dimension(window.height);
    let x = match horizontal_alignment {
        Alignment::Minimum => width,
        Alignment::Center => (window.width / 2.0) + (width / 2.0),
        Alignment::Maximum => window.width,
    };
    let y = match vertical_alignment {
        Alignment::Minimum => window.height - height,
        Alignment::Center => (window.height / 2.0) + (height / 2.0),
        Alignment::Maximum => window.height,
    };
    Vertex2D { x, y }
}

fn client_to_sized_rectangle(window: &Dimension2D, vertical_sizing: &Sizing, horizontal_sizing: &Sizing, vertical_alignment: &Alignment, horizontal_alignment: &Alignment) -> Rectangle2D {
    let origin = client_to_origin(window, vertical_sizing, horizontal_sizing, vertical_alignment, horizontal_alignment);
    let antipode = client_to_antipode(window, vertical_sizing, horizontal_sizing, vertical_alignment, horizontal_alignment);
    Rectangle2D { origin, antipode }
}
