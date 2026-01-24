use crate::graphics::geometry::dim::Dimension2D;
use crate::graphics::geometry::point::p3d::Point3D;
use crate::logger::log;
use crate::logger::log_level::LogLevel;

pub struct Camera {
    pub position: Point3D<f32>,     // position of camera, world coords
    pub forward: Point3D<f32>,      // direction camera is looking
    pub up: Point3D<f32>,           // up from camera

    pub pitch: f32,                 // x-axis rotation
    pub yaw: f32,                   // y-axis rotation

    pub width: f32,                 // window width
    pub height: f32,                // window height

    pub near: f64,                  // 3d near clipping plane
    pub far: f64,                   // 3d far clipping plane
}

impl Camera {
    pub fn new(screen: &Dimension2D<f32>) -> Camera {
        Camera {
            width: screen.width,
            height: screen.height,
            ..Default::default()
        }
    }

    pub fn update_screen(&mut self, screen: &Dimension2D<f32>) {
        self.width = screen.width;
        self.height = screen.height;
        log(LogLevel::Info, &|| String::from(format!("updated screen: width={}, height={}", self.width, self.height)));
    }

    pub fn aspect(&self) -> f64 {
        (self.width / self.height) as f64
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            position: Point3D::new(0.0, 0.0, 1.5),
            forward: Point3D::origin(),
            up: Point3D::origin(),

            pitch: 0.0,
            yaw: 0.0,

            width: 800.0,
            height: 600.0,

            near: 0.01,
            far: 500.0,
        }
    }
}
