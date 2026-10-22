use crate::geometry::angle::degrees::Degrees;
use crate::geometry::orient::matrix::m4x4::Matrix4x4;

pub mod matrix;
pub mod movement;
pub mod quaternion;

#[derive(Clone)]
pub struct Orientation {
    pub transform: Matrix4x4, // orientation; c1=right(x), c2=up(y), c3=forward(z/normal), c4=position

    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
    pub pitch: Degrees, // up/down, about x
    pub yaw: Degrees,   // left/right; about y
    pub roll: Degrees,  // side/side, about z
    pub pitch_delta: Degrees,
    pub yaw_delta: Degrees,
    pub roll_delta: Degrees,
}

pub struct OrientationBuilder {
    the_position: Option<Matrix4x4>,
    the_x_scale: Option<f32>,
    the_y_scale: Option<f32>,
    the_z_scale: Option<f32>,
    the_pitch: Option<Degrees>,
    the_yaw: Option<Degrees>,
    the_roll: Option<Degrees>,
}

impl Orientation {
    pub fn new(
        position: Matrix4x4,
        x_scale: f32,
        y_scale: f32,
        z_scale: f32,
        pitch: Degrees,
        yaw: Degrees,
        roll: Degrees,
    ) -> Orientation {
        Orientation { transform: position, x_scale, y_scale, z_scale, pitch, yaw, roll, pitch_delta: Degrees::zero(), yaw_delta: Degrees::zero(), roll_delta: Degrees::zero(),}
    }

    pub fn camera_default() -> Orientation {
        Orientation {
            transform: Matrix4x4 {
                r1c4: 0.0,
                r2c4: 0.0,
                r3c4: 1.5,
                ..Default::default()
            },
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            pitch: Degrees::zero(),
            yaw: Degrees::zero(),
            roll: Degrees::zero(),
            pitch_delta: Degrees::zero(),
            yaw_delta: Degrees::zero(),
            roll_delta: Degrees::zero(),
        }
    }
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation {
            transform: Matrix4x4::default(),
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            pitch: Degrees::zero(),
            yaw: Degrees::zero(),
            roll: Degrees::zero(),
            pitch_delta: Degrees::zero(),
            yaw_delta: Degrees::zero(),
            roll_delta: Degrees::zero(),
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
            the_roll: None,
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
            transform: self.the_position.unwrap_or_else(|| Matrix4x4::default()),
            x_scale: self.the_x_scale.unwrap_or_else(|| 1.0),
            y_scale: self.the_y_scale.unwrap_or_else(|| 1.0),
            z_scale: self.the_z_scale.unwrap_or_else(|| 1.0),
            pitch: self.the_pitch.unwrap_or_else(|| Degrees::zero()),
            yaw: self.the_yaw.unwrap_or_else(|| Degrees::zero()),
            roll: self.the_roll.unwrap_or_else(|| Degrees::zero()),
            pitch_delta:Degrees::zero(),
            yaw_delta:Degrees::zero(),
            roll_delta:Degrees::zero(),
        }
    }
}
