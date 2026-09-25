use crate::geometry::dim::Dimension2D;
use crate::geometry::orient::matrix::m4x4::Matrix4x4;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

///
/// The screen projection.
///
pub struct Projection {
    pub width: f32,  // window width
    pub height: f32, // window height

    pub near: f32, // 3d near clipping plane
    pub far: f32,  // 3d far clipping plane

    pub fov: f32, // field of view
}

impl Projection {
    pub(crate) fn new(width: f32, height: f32) -> Projection {
        Projection {
            width,
            height,
            ..Default::default()
        }
    }

    pub(crate) fn update_screen(&mut self, dimension: &Dimension2D) {
        self.width = dimension.width;
        self.height = dimension.height;
        log(LogLevel::Info, &|| { String::from(format!("updated screen: width={}, height={}", self.width as f64, self.height as f64)) });
    }
}

impl Projection {
    pub(crate) fn to_aspect(&self) -> f32 {
        self.width / self.height
    }

    pub(crate) fn to_matrix(&self) -> Matrix4x4 {
        let f = 1.0 / (self.fov / 2.0).tan();
        let aspect = self.to_aspect();
        Matrix4x4 {
            r1c1: f / aspect,
            r2c1: 0.0,
            r3c1: 0.0,
            r4c1: 0.0,

            r1c2: 0.0,
            r2c2: f,
            r3c2: 0.0,
            r4c2: 0.0,

            r1c3: 0.0,
            r2c3: 0.0,
            r3c3: (self.far + self.near) / (self.near - self.far),
            r4c3: -1.0,

            r1c4: 0.0,
            r2c4: 0.0,
            r3c4: (2.0 * self.far * self.near) / (self.near - self.far),
            r4c4: 0.0,
        }
    }
}

impl Default for Projection {
    fn default() -> Projection {
        Projection {
            width: 800.0,
            height: 600.0,

            near: 0.01,
            far: 500.0,

            fov: 45.0,
        }
    }
}
