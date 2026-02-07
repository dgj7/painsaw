use crate::graphics::geometry::orient::Orientation;
use crate::graphics::geometry::dim::Dimension2D;
use crate::logger::log;
use crate::logger::log_level::LogLevel;

pub struct Camera {
    pub orientation: Orientation,

    pub width: f32,                   // window width
    pub height: f32,                  // window height

    pub near: f32,                    // 3d near clipping plane
    pub far: f32,                     // 3d far clipping plane
}

impl Camera {
    pub fn new(screen: &Dimension2D) -> Camera {
        Camera {
            width: screen.width,
            height: screen.height,
            ..Default::default()
        }
    }
}

impl Camera {
    pub fn aspect(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn update_screen(&mut self, screen: &Dimension2D) {
        self.width = screen.width;
        self.height = screen.height;
        log(LogLevel::Info, &|| String::from(format!("updated screen: width={}, height={}", self.width as f64, self.height as f64)));
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            orientation: Orientation::camera_default(),

            width: 800.0,
            height: 600.0,

            near: 0.01,
            far: 500.0,
        }
    }
}
