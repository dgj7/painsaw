use crate::graphics::geometry::dim::Dimension2D;
use crate::graphics::geometry::orient::Orientation;
use crate::graphics::geometry::projection::Projection;

pub struct Camera {
    pub orientation: Orientation,
    pub projection: Projection,
}

impl Camera {
    pub fn new(screen: &Dimension2D) -> Camera {
        Camera {
            projection: Projection::new(screen),
            ..Default::default()
        }
    }
}

impl Camera {
    pub fn aspect(&self) -> f32 {
        self.projection.to_aspect()
    }

    pub fn update_screen(&mut self, screen: &Dimension2D) {
        self.projection.update_screen(screen)
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            orientation: Orientation::camera_default(),
            projection: Projection::default(),
        }
    }
}
