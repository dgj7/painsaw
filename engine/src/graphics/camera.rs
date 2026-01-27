use crate::graphics::geometry::orient::Orientation;
use num_traits::Float;
use crate::graphics::geometry::dim::Dimension2D;
use crate::logger::log;
use crate::logger::log_level::LogLevel;

pub struct Camera<F: Float> {
    pub orientation: Orientation<F>,

    pub width: F,                   // window width
    pub height: F,                  // window height

    pub near: F,                    // 3d near clipping plane
    pub far: F,                     // 3d far clipping plane
}

impl<F: Float> Camera<F> {
    pub fn new(screen: &Dimension2D<f32>) -> Camera<F> {
        Camera {
            width: F::from(screen.width).unwrap(),
            height: F::from(screen.height).unwrap(),
            ..Default::default()
        }
    }
}

impl<F: Float> Camera<F> {
    pub fn aspect(&self) -> f64 {
        (F::from(self.width).unwrap() / F::from(self.height).unwrap()).to_f64().unwrap()
    }

    pub fn update_screen(&mut self, screen: &Dimension2D<f32>) {
        self.width = F::from(screen.width).unwrap();
        self.height = F::from(screen.height).unwrap();
        log(LogLevel::Info, &|| String::from(format!("updated screen: width={}, height={}", self.width.to_f64().unwrap(), self.height.to_f64().unwrap())));
    }
}

impl<F: Float> Default for Camera<F> {
    fn default() -> Camera<F> {
        Camera {
            orientation: Orientation::camera_default(),

            width: F::from(800.0).unwrap(),
            height: F::from(600.0).unwrap(),

            near: F::from(0.01).unwrap(),
            far: F::from(500.0).unwrap(),
        }
    }
}
