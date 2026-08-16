use crate::geometry::orient::matrix::m4x4::Matrix4x4;

pub mod matrix;
pub mod quaternion;
pub mod movement;

#[derive(Clone)]
pub struct Orientation {
    pub position: Matrix4x4, // orientation; c1=right(x), c2=up(y), c3=forward(z/normal), c4=position
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
    pub pitch: f32,
    pub yaw: f32,
}

pub struct OrientationBuilder {
    the_position: Option<Matrix4x4>,
    the_x_scale: Option<f32>,
    the_y_scale: Option<f32>,
    the_z_scale: Option<f32>,
    the_pitch: Option<f32>,
    the_yaw: Option<f32>,
}

impl Orientation {
    pub fn new(position: Matrix4x4, x_scale: f32, y_scale: f32, z_scale: f32, pitch: f32,  yaw: f32) -> Orientation {
        Orientation {
            position,
            x_scale,
            y_scale,
            z_scale,
            pitch,
            yaw,
        }
    }

    pub fn camera_default() -> Orientation {
        Orientation {
            position: Matrix4x4 {
                c4r1: 0.0,
                c4r2: 0.0,
                c4r3: 1.5,
                ..Default::default()
            },
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation {
            position: Matrix4x4::default(),
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl OrientationBuilder {
    pub fn new() -> OrientationBuilder {
        OrientationBuilder {
            the_position: None,
            the_x_scale: None,
            the_y_scale: None,
            the_z_scale: None,
            the_pitch: None,
            the_yaw: None,
        }
    }

    pub fn with_position(mut self, position: Matrix4x4) -> OrientationBuilder {
        self.the_position = Some(position);
        self
    }

    pub fn with_x_scale(mut self, scale: f32) -> OrientationBuilder {
        self.the_x_scale = Some(scale);
        self
    }

    pub fn with_y_scale(mut self, scale: f32) -> OrientationBuilder {
        self.the_y_scale = Some(scale);
        self
    }

    pub fn with_z_scale(mut self, scale: f32) -> OrientationBuilder {
        self.the_z_scale = Some(scale);
        self
    }

    pub fn build(self) -> Orientation {
        Orientation {
            position: self.the_position.unwrap_or_else(|| Matrix4x4::default()),
            x_scale: self.the_x_scale.unwrap_or_else(|| 1.0),
            y_scale: self.the_y_scale.unwrap_or_else(|| 1.0),
            z_scale: self.the_z_scale.unwrap_or_else(|| 1.0),
            pitch: self.the_pitch.unwrap_or_else(|| 0.0),
            yaw: self.the_yaw.unwrap_or_else(|| 0.0),
        }
    }
}
