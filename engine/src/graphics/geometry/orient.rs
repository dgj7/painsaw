use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;
use num_traits::Float;
use crate::config::EngineConfig;
use crate::graphics::geometry::primitive::point::p3d::Point3D;

pub mod euler;
pub mod matrix;
pub mod quaternion;

pub struct Orientation<F: Float> {
    pub position: Matrix4x4<F>,  // orientation; c1=right(x), c2=up(y), c3=forward(z/normal), c4=position
    pub x_scale: F,
    pub y_scale: F,
    pub z_scale: F,
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
        let change = Point3D::new_mult_scalar(&Point3D::new_mult_scalar(&forward, config.movement.forward_speed), delta_time);
        let updated = Point3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_backward(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let forward = self.position.column_major_z_forward();
        let position = self.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Point3D::new_mult_scalar(&Point3D::new_mult_scalar(&forward, config.movement.forward_speed), delta_time);
        let updated = Point3D::new_add(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_left(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let right = self.position.column_major_x_right();
        let position = self.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Point3D::new_mult_scalar(&Point3D::new_mult_scalar(&right, config.movement.forward_speed), delta_time);
        let updated = Point3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        self.position.column_major_update_position(&updated);
    }

    pub fn move_right(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let right = self.position.column_major_x_right();
        let position = self.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Point3D::new_mult_scalar(&Point3D::new_mult_scalar(&right, config.movement.forward_speed), delta_time);
        let updated = Point3D::new_add(&position, &change);

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
