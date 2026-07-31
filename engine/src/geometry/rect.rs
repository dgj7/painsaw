use windows::Win32::Foundation::RECT;
use crate::geometry::primitive::v2d::Vertex2D;

#[derive(Clone, Debug)]
pub struct Rectangle2D {
    pub top_left: Vertex2D,
    pub bottom_right: Vertex2D,
}

impl Rectangle2D {
    #[cfg(target_os="windows")]
    pub fn new(rect: RECT) -> Rectangle2D {
        Rectangle2D {
            top_left: Vertex2D {
                x: rect.left as f32,
                y: rect.top as f32,
            },
            bottom_right: Vertex2D {
                x: rect.right as f32,
                y: rect.bottom as f32,
            }
        }
    }
}
