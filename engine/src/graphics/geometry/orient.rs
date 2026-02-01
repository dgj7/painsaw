use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;
use num_traits::Float;
use crate::config::EngineConfig;
use crate::graphics::geometry::primitive::v3d::Vertex3D;

pub mod euler;
pub mod matrix;
pub mod quaternion;

#[derive(Clone)]
pub struct Orientation<F: Float> {
    pub position: Matrix4x4<F>,  // orientation; c1=right(x), c2=up(y), c3=forward(z/normal), c4=position
    pub x_scale: F,
    pub y_scale: F,
    pub z_scale: F,
}

pub struct OrientationBuilder<F: Float> {
    the_position: Option<Matrix4x4<F>>,
    the_x_scale: Option<F>,
    the_y_scale: Option<F>,
    the_z_scale: Option<F>,
}

impl<F: Float> Orientation<F> {
    pub fn new(position: Matrix4x4<F>, x_scale: F, y_scale: F, z_scale: F) -> Orientation<F> {
        Orientation {
            position,
            x_scale,
            y_scale,
            z_scale,
        }
    }

    pub fn camera_default() -> Orientation<F> {
        Orientation {
            position: Matrix4x4 {
                c4r1: F::zero(),
                c4r2: F::zero(),
                c4r3: F::from(1.5).unwrap(),
                ..Default::default()
            },
            x_scale: F::from(1.0).unwrap(),
            y_scale: F::from(1.0).unwrap(),
            z_scale: F::from(1.0).unwrap(),
        }
    }
}

impl<F: Float> Orientation<F> {
    pub fn pitch(&self) -> F {
        // todo
        F::zero()
    }

    pub fn yaw(&self) -> F {
        // todo
        F::zero()
    }
}

impl<F: Float> Orientation<F> {
    pub fn move_forward(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let forward = self.position.column_major_z_forward();
        let position = self.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), delta_time);
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_backward(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let forward = self.position.column_major_z_forward();
        let position = self.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), delta_time);
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_left(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let right = self.position.column_major_x_right();
        let position = self.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), delta_time);
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_right(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let right = self.position.column_major_x_right();
        let position = self.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), delta_time);
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }
}

impl<F: Float> Default for Orientation<F> {
    fn default() -> Orientation<F> {
        Orientation {
            position: Matrix4x4::default(),
            x_scale: F::one(),
            y_scale: F::one(),
            z_scale: F::one(),
        }
    }
}

impl<F: Float> OrientationBuilder<F> {
    pub fn new() -> OrientationBuilder<F> {
        OrientationBuilder {
            the_position: None,
            the_x_scale: None,
            the_y_scale: None,
            the_z_scale: None,
        }
    }
    
    pub fn with_position(mut self, position: Matrix4x4<F>) -> OrientationBuilder<F> {
        self.the_position = Some(position);
        self
    }
    
    pub fn with_x_scale(mut self, scale: F) -> OrientationBuilder<F> {
        self.the_x_scale = Some(scale);
        self
    }
    
    pub fn with_y_scale(mut self, scale: F) -> OrientationBuilder<F> {
        self.the_y_scale = Some(scale);
        self
    }
    
    pub fn with_z_scale(mut self, scale: F) -> OrientationBuilder<F> {
        self.the_z_scale = Some(scale);
        self
    }
    
    pub fn build(self) -> Orientation<F> {
        Orientation {
            position: self.the_position.unwrap_or_else(|| Matrix4x4::default()),
            x_scale: self.the_x_scale.unwrap_or_else(|| F::one()),
            y_scale: self.the_y_scale.unwrap_or_else(|| F::one()),
            z_scale: self.the_z_scale.unwrap_or_else(|| F::one()),
        }
    }
}
