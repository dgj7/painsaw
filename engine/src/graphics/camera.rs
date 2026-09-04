use crate::geometry::orient::Orientation;
use crate::geometry::projection::Projection;
use crate::input::screen::ScreenState;

pub struct Camera {
    pub screen: ScreenState,
    pub orientation: Orientation,
    pub projection: Projection,
}

impl Camera {
    pub fn new(screen: ScreenState) -> Camera {
        let width = screen.current_client_dimensions.width;
        let height = screen.current_client_dimensions.height;
        Camera {
            screen,
            orientation: Orientation::default(),
            projection: Projection::new(width, height),
        }
    }
}

impl Camera {
    pub fn aspect(&self) -> f32 {
        self.projection.to_aspect()
    }

    pub fn update_screen(&mut self) {
        self.projection.update_screen(&self.screen.current_client_dimensions)
    }
}
