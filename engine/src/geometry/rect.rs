use crate::geometry::primitive::v2d::Vertex2D;
use windows::Win32::Foundation::RECT;
use crate::graphics::storage::qt::attrib::layout::Layout;

#[derive(Clone, Debug)]
pub struct Rectangle2D {
    pub origin: Vertex2D,
    pub antipode: Vertex2D,
}

impl Rectangle2D {
    #[cfg(target_os = "windows")]
    pub fn new(rect: RECT) -> Rectangle2D {
        Rectangle2D {
            origin: Vertex2D {
                x: rect.left as f32,
                y: rect.top as f32,
            },
            antipode: Vertex2D {
                x: rect.right as f32,
                y: rect.bottom as f32,
            },
        }
    }

    pub fn contains_pt_inclusive(&self, point: &Vertex2D) -> bool {
        let x_ok = point.x >= self.origin.x && point.x <= self.antipode.x;
        let y_ok = point.y >= self.origin.y && point.y <= self.antipode.y;
        x_ok && y_ok
    }

    pub fn contains_pt_exclusive(&self, point: &Vertex2D) -> bool {
        let x_ok = point.x > self.origin.x && point.x < self.antipode.x;
        let y_ok = point.y > self.origin.y && point.y < self.antipode.y;
        x_ok && y_ok
    }

    pub fn contains_rect_inclusive(&self, rectangle: &Rectangle2D) -> bool {
        self.contains_pt_inclusive(&rectangle.origin) && self.contains_pt_inclusive(&rectangle.antipode)
    }

    pub fn contains_rect_exclusive(&self, rectangle: &Rectangle2D) -> bool {
        self.contains_pt_exclusive(&rectangle.origin) && self.contains_pt_exclusive(&rectangle.antipode)
    }

    pub fn to_width(&self) -> f32 {
        self.antipode.x - self.origin.x
    }

    pub fn to_height(&self) -> f32 {
        self.antipode.y - self.origin.y
    }

    ///
    /// get the "other" dimension for a layout.
    ///
    /// vertical layouts always consume all the vertical space; we only
    /// need to know how much width it consumes.
    ///
    /// horizontal layouts always consume all the horizontal space; we only
    /// need to know how much height it consumes.
    ///
    pub fn to_other_dimension(&self, layout: &Layout) -> f32 {
        match layout {
            Layout::Horizontal { .. } => self.to_height(),
            Layout::Vertical { .. } => self.to_width(),
        }
    }

    ///
    /// convert to four vertices.
    ///
    pub fn to_vertices(&self) -> Vec<Vertex2D> {
        vec!(
            self.origin.clone(),
            Vertex2D::new(self.antipode.x, self.origin.y),
            self.antipode.clone(),
            Vertex2D::new(self.origin.x, self.antipode.y),
        )
    }
}
