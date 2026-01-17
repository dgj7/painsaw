use crate::geometry::dim::Dimension2D;
use crate::geometry::point::p3d::Point3D;
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
}

impl Camera {
    pub fn new(screen: &Dimension2D<f32>) -> Camera {
        Camera {
            position: Point3D::new(0.0, 0.0, 1.5),
            forward: Point3D::origin(),
            up: Point3D::origin(),
            pitch: 0.0,
            yaw: 0.0,
            width: screen.width,
            height: screen.height,
        }
    }

    pub fn update_screen(&mut self, screen: &Dimension2D<f32>) {
        self.width = screen.width;
        self.height = screen.height;
        log(LogLevel::Info, &|| String::from(format!("updated screen: width={}, height={}", self.width, self.height)));
    }
}
