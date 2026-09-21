use crate::geometry::primitive::mode::PolygonMode;
use crate::geometry::primitive::prim2d::Primitive2DBuilder;
use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::primitive::PrimitiveType;
use crate::geometry::rect::Rectangle2D;
use crate::graphics::color::Color;
use crate::graphics::storage::g2d::m2d::Model2D;
use crate::graphics::storage::ui::view::attrib::assembled::Assembled;
use crate::graphics::texture::t2d::Texture2DBuilder;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use crate::support::text::{text_2d_image, TextConfig, Typeface};

///
/// a widget is any control that can be clicked on screen.
///
pub struct Widget {
    click_action: fn(pt: &Vertex2D),    /* how to handle when a click has registered */
    text: Option<String>,

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
    fn reassemble(&self, _debug: bool, model: &mut Model2D, rectangle: &Rectangle2D) {
        /* draw the button's outline and filling */
        model.primitives
            .push(Primitive2DBuilder::new()
                .with_mode(PolygonMode::Fill)
                .with_color(Color::RED)
                .with_type(PrimitiveType::Cube { thickness: 1.0 })
                .with_vertex(Vertex2D::new(rectangle.origin.x + self.padding, rectangle.origin.y + self.padding))         // top left (origin)
                .with_vertex(Vertex2D::new(rectangle.antipode.x - self.padding, rectangle.origin.y + self.padding))       // top right
                .with_vertex(Vertex2D::new(rectangle.antipode.x - self.padding, rectangle.antipode.y - self.padding))     // bottom right (antipode)
                .with_vertex(Vertex2D::new(rectangle.origin.x + self.padding, rectangle.antipode.y-self.padding))         // bottom left
                .build());

        /* potentially render text if any has been provided */
        if let Some(t) = &self.text {
            /* do some additional computations so we can figure out where the text goes */
            let height = rectangle.antipode.y - rectangle.origin.y;
            let text = text_2d_image(TextConfig { typeface: Typeface::Generic, foreground: Color::WHITE, background: Color::TRANSPARENT}, || String::from(t));
            let scale = (height / text.height as f32) / 3.0;
            let scaled_text_height = text.height as f32 * scale;

            /* add text to the button */
            model.textures
                .push(Texture2DBuilder::new()
                    .with_x(rectangle.origin.x + 10.0)
                    .with_y(rectangle.origin.y + (height / 2.0) - (scaled_text_height / 2.0))
                    .with_scale(scale)
                    .with_image(text)
                    .build());
        }

        /* do some logging */
        log(LogLevel::Debug, &|| format!("widget assembled: origin=({},{}),antipode=({},{})", rectangle.origin.x, rectangle.origin.y, rectangle.antipode.x, rectangle.antipode.y));
    }
}

///
/// fluent builder for easier creation of widgets.
/// 
pub struct WidgetBuilder {
    the_click_action: Option<fn(pt: &Vertex2D)>,
    the_text: Option<String>,

    the_padding: Option<f32>,
}

impl WidgetBuilder {
    pub fn new() -> Self {
        WidgetBuilder {
            the_click_action: None,
            the_text: None,

            the_padding: None,
        }
    }

    pub fn with_click_action(mut self, click_action: fn(pt: &Vertex2D)) -> Self {
        self.the_click_action = Some(click_action);
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.the_text = Some(String::from(text));
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.the_padding = Some(padding);
        self
    }

    pub fn build(self) -> Widget {
        Widget {
            click_action: self.the_click_action.unwrap_or_else(|| |_vertex|{}),
            text: self.the_text,

            padding: self.the_padding.unwrap_or_else(|| 10.0),
        }
    }
}
