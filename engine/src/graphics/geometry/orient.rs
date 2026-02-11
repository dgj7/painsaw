use crate::config::EngineConfig;
use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;
use crate::graphics::geometry::primitive::v3d::Vertex3D;
use crate::graphics::timing::EngineTiming;

pub mod euler;
pub mod matrix;
pub mod quaternion;

#[derive(Clone)]
pub struct Orientation {
    pub position: Matrix4x4, // orientation; c1=right(x), c2=up(y), c3=forward(z/normal), c4=position
    pub x_scale: f32,
    pub y_scale: f32,
    pub z_scale: f32,
}

pub struct OrientationBuilder {
    the_position: Option<Matrix4x4>,
    the_x_scale: Option<f32>,
    the_y_scale: Option<f32>,
    the_z_scale: Option<f32>,
}

impl Orientation {
    pub fn new(position: Matrix4x4, x_scale: f32, y_scale: f32, z_scale: f32) -> Orientation {
        Orientation {
            position,
            x_scale,
            y_scale,
            z_scale,
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
        }
    }
}

impl Orientation {
    pub fn pitch(&self) -> f32 {
        // todo
        0.0
    }

    pub fn yaw(&self) -> f32 {
        // todo
        0.0
    }
}

impl Orientation {
    pub fn move_forward(&mut self, config: &EngineConfig, timing: &EngineTiming) {
        /* gather necessary variables */
        let forward = self.position.column_major_z_forward();
        let position = self.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32);
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_backward(&mut self, config: &EngineConfig, timing: &EngineTiming) {
        /* gather necessary variables */
        let forward = self.position.column_major_z_forward();
        let position = self.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32);
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_left(&mut self, config: &EngineConfig, timing: &EngineTiming) {
        /* gather necessary variables */
        let right = self.position.column_major_x_right();
        let position = self.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32);
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_right(&mut self, config: &EngineConfig, timing: &EngineTiming) {
        /* gather necessary variables */
        let right = self.position.column_major_x_right();
        let position = self.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32);
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation {
            position: Matrix4x4::default(),
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
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
        }
    }
}
