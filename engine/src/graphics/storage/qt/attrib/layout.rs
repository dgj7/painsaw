use crate::geometry::primitive::v2d::Vertex2D;
use crate::geometry::rect::Rectangle2D;
use crate::graphics::storage::qt::attrib::sizing::Sizing;
use crate::graphics::storage::qt::panel::Panel;
use crate::graphics::storage::qt::sr::SizingRequest;

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
    pub fn determine_next(&self, container: &Rectangle2D, sizing: &SizingRequest, padding: &Sizing) -> Rectangle2D {
        match self {
            Layout::Horizontal => horizontal(container, sizing, padding),
            Layout::Vertical => vertical(container, sizing, padding),
        }
    }

    ///
    /// subtract some amount from a rectangle, with the subtraction area
    /// dependent on the layout type.
    ///
    pub fn subtract(&self, rectangle: &mut Rectangle2D, subtrahend: &Rectangle2D) {
        match self {
            Layout::Horizontal => { rectangle.origin.x = rectangle.origin.x + subtrahend.antipode.x; },
            Layout::Vertical => { rectangle.origin.y = rectangle.origin.y + subtrahend.antipode.y; },
        }
    }
}

fn horizontal(container: &Rectangle2D, sizing: &SizingRequest, padding: &Sizing) -> Rectangle2D {
    /* get the (parent) container's dimensions */
    let container_x = container.to_x_diff();
    let container_y = container.to_y_diff();

    /* determine the dimension used for padding  */
    let pad = padding.from_client_to_dimension(container.antipode.x - container.origin.x);

    /* determine the new x and y dimensions */
    let new_horiz_dim = sizing.horizontal_sizing.from_client_to_dimension(container_x);
    let new_verti_dim = sizing.vertical_sizing.from_client_to_dimension(container_y);

    Rectangle2D {
        origin: Vertex2D {
            x: container.origin.x + pad,
            y: container.origin.y + pad,
        },
        antipode: Vertex2D {
            x: container.origin.x + pad + new_horiz_dim,
            y: container.origin.y + pad + new_verti_dim,
        }
    }
}

fn vertical(container: &Rectangle2D, sizing: &SizingRequest, padding: &Sizing) -> Rectangle2D {
    Rectangle2D {
        origin: Vertex2D {
            x: 0.0,
            y: 0.0,
        },
        antipode: Vertex2D {
            x: 0.0,
            y: 0.0,
        }
    }
}
