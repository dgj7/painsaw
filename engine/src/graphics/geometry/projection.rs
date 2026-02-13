use crate::graphics::geometry::dim::Dimension2D;
use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;
use crate::logger::log;
use crate::logger::log_level::LogLevel;

///
/// The screen projection.
///
pub struct Projection {
    pub width: f32,     // window width
    pub height: f32,    // window height

    pub near: f32,      // 3d near clipping plane
    pub far: f32,       // 3d far clipping plane

    pub fov: f32,       // field of view
}

impl Projection {
    pub(crate) fn new(dimension: &Dimension2D) -> Projection {
        Projection {
            width: dimension.width,
            height: dimension.height,
            ..Default::default()
        }
    }
    
    pub(crate) fn update_screen(&mut self, dimension: &Dimension2D) {
        self.width = dimension.width;
        self.height = dimension.height;
        log(LogLevel::Info, &|| String::from(format!("updated screen: width={}, height={}", self.width as f64, self.height as f64)));
    }
}

impl Projection {
    pub(crate) fn to_aspect(&self) -> f32 {
        self.width / self.height
    }

    #[allow(unused)] // todo: remove
    pub(crate) fn to_matrix(&self) -> Matrix4x4 {
        let f = 1.0 / (self.fov / 2.0).tan();
        let aspect = self.to_aspect();
        Matrix4x4 {
            c1r1: f / aspect,
            c1r2: 0.0,
            c1r3: 0.0,
            c1r4: 0.0,

            c2r1: 0.0,
            c2r2: f,
            c2r3: 0.0,
            c2r4: 0.0,


            c3r1: 0.0,
            c3r2: 0.0,
            c3r3: (self.far + self.near) / (self.near - self.far),
            c3r4: -1.0,

            c4r1: 0.0,
            c4r2: 0.0,
            c4r3: (2.0 * self.far * self.near) / (self.near - self.far),
            c4r4: 0.0,
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
