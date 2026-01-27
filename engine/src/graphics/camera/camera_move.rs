use num_traits::Float;
use crate::config::EngineConfig;
use crate::graphics::camera::Camera;
use crate::graphics::geometry::primitive::point::p3d::Point3D;

impl<F: Float> Camera<F> {
    pub fn move_forward(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let forward = self.orientation.column_major_z_forward();
        let position = self.orientation.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Point3D::new_mult_scalar(&Point3D::new_mult_scalar(&forward, config.movement.forward_speed), delta_time);
        let updated = Point3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        self.orientation.column_major_update_position(&updated);
    }

    pub fn move_backward(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let forward = self.orientation.column_major_z_forward();
        let position = self.orientation.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Point3D::new_mult_scalar(&Point3D::new_mult_scalar(&forward, config.movement.forward_speed), delta_time);
        let updated = Point3D::new_add(&position, &change);

        /* update the orientation matrix */
        self.orientation.column_major_update_position(&updated);
    }

    pub fn move_left(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let right = self.orientation.column_major_x_right();
        let position = self.orientation.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Point3D::new_mult_scalar(&Point3D::new_mult_scalar(&right, config.movement.forward_speed), delta_time);
        let updated = Point3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        self.orientation.column_major_update_position(&updated);
    }

    pub fn move_right(&mut self, config: &EngineConfig<F>, delta_time: F) {
        /* gather necessary variables */
        let right = self.orientation.column_major_x_right();
        let position = self.orientation.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Point3D::new_mult_scalar(&Point3D::new_mult_scalar(&right, config.movement.forward_speed), delta_time);
        let updated = Point3D::new_add(&position, &change);

        /* update the orientation matrix */
        self.orientation.column_major_update_position(&updated);
    }
}
