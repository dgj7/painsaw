use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::rect::Rectangle2D;
use crate::graphics::storage::qt::view::attrib::sizing::Sizing;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

///
/// [Panel] sizer specification, to determine how elements are laid out within.
///
pub enum Layout {
    Horizontal,
    Vertical,
}

impl Layout {
    ///
    /// determine a rectangle for the next element.
    ///
    pub fn determine_next(&self, container: &Rectangle2D, remainder: &Rectangle2D, sizing: &Sizing) -> Rectangle2D {
        match self {
            Layout::Horizontal { .. } => horizontal(container, remainder, sizing),
            Layout::Vertical { .. } => vertical(container, remainder, sizing),
        }
    }

    ///
    /// subtract some amount from a rectangle, with the subtraction area
    /// dependent on the layout type.
    ///
    pub fn subtract(&self, container: &mut Rectangle2D, next: &Rectangle2D) {
        match self {
            Layout::Horizontal { .. } => { container.origin.x = container.origin.x + next.to_width(); },
            Layout::Vertical { .. } => { container.origin.y = container.origin.y + next.to_height(); },
        }
    }
}

///
/// "private" function to compute the next rectangle in a horizontal container with the given sizing.
///
fn horizontal(container: &Rectangle2D, remainder: &Rectangle2D, sizing: &Sizing) -> Rectangle2D {
    let element_height = container.to_height();                                                /* get the "other" constant dimension, that's the same for everything in this container */
    let element_width = sizing.container_to_dimension(container.to_width());                   /* compute the element's variable dimension */
    next(remainder, element_width, element_height)                                                  /* compute the origin and antipode based on the above */
}

///
/// "private" function to compute the next rectangle in a vertical container with the given sizing.
///
fn vertical(container: &Rectangle2D, remainder: &Rectangle2D, sizing: &Sizing) -> Rectangle2D {
    let element_width = container.to_width();                                                  /* get the "other" constant dimension, that's the same for everything in this container */
    let element_height = sizing.container_to_dimension(container.to_height());                 /* compute the element's variable dimension */
    next(remainder, element_width, element_height)                                                  /* compute the origin and the antipode based on the above */
}

///
/// "private" function to compute the next rectangle given the container and the next rectangle's dimensions.
///
fn next(container: &Rectangle2D, width: f32, height: f32) -> Rectangle2D {
    let origin = Vertex2D {
        x: container.origin.x,
        y: container.origin.y
    };
    let antipode = Vertex2D {
        x: container.origin.x + width,
        y: container.origin.y + height
    };
    log(LogLevel::Debug, &|| format!("Layout::Horizontal: ch/eh={},cw={},ew={},origin=({},{}),antipode=({},{})", height, container.to_width(), width, origin.x, origin.y, antipode.x, antipode.y));
    Rectangle2D { origin, antipode }
}
