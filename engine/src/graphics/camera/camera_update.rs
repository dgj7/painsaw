use num_traits::Float;
use crate::graphics::camera::Camera;
use crate::graphics::geometry::dim::Dimension2D;
use crate::logger::log;
use crate::logger::log_level::LogLevel;

impl<F: Float> Camera<F> {
    pub fn update_screen(&mut self, screen: &Dimension2D<f32>) {
        self.width = F::from(screen.width).unwrap();
        self.height = F::from(screen.height).unwrap();
        log(LogLevel::Info, &|| String::from(format!("updated screen: width={}, height={}", self.width.to_f64().unwrap(), self.height.to_f64().unwrap())));
    }
}
