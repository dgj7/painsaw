use crate::config::EngineConfig;
use crate::game::movement::move_strategy::MovementStrategy;
use crate::geometry::primitive::v3d::Vertex3D;
use crate::graphics::camera::Camera;
use crate::support::timing::EngineTiming;

pub struct FpsMoveStrategy {
    pub forward: bool,
    pub left: bool,
    pub right: bool,
    pub backward: bool,
}

impl MovementStrategy for FpsMoveStrategy {
    fn move_camera(&mut self, camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
        if self.forward {
            /* gather necessary variables */
            let forward = camera.orientation.position.column_major_z_forward();
            let position = camera.orientation.position.column_major_position();

            /* compute change (forward * speed * delta_time), then update position */
            let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32);
            let updated = Vertex3D::new_subtract(&position, &change);

            /* update the orientation matrix */
            camera.orientation.position.column_major_update_position(&updated);
        }
        
        if self.backward {
            /* gather necessary variables */
            let forward = camera.orientation.position.column_major_z_forward();
            let position = camera.orientation.position.column_major_position();

            /* compute change (forward * speed * delta_time), then update position */
            let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32);
            let updated = Vertex3D::new_add(&position, &change);

            /* update the orientation matrix */
            camera.orientation.position.column_major_update_position(&updated);
        }
        
        if self.left {
            /* gather necessary variables */
            let right = camera.orientation.position.column_major_x_right();
            let position = camera.orientation.position.column_major_position();

            /* compute change (right * speed * delta_time), then update position */
            let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32);
            let updated = Vertex3D::new_subtract(&position, &change);

            /* update the orientation matrix */
            camera.orientation.position.column_major_update_position(&updated);
        }
        
        if self.right {
            /* gather necessary variables */
            let right = camera.orientation.position.column_major_x_right();
            let position = camera.orientation.position.column_major_position();

            /* compute change (right * speed * delta_time), then update position */
            let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32);
            let updated = Vertex3D::new_add(&position, &change);

            /* update the orientation matrix */
            camera.orientation.position.column_major_update_position(&updated);
        }
    }
}
