use crate::config::EngineConfig;
use crate::geometry::angle::radians::Radians;
use crate::geometry::orient::matrix::m3x3::column_major::from_pitch_yaw_roll;
use crate::geometry::orient::matrix::rotate;
use crate::geometry::primitive::v3d::Vertex3D;
use crate::graphics::camera::Camera;
use crate::input::mouse::md::MouseDelta;
use crate::support::timing::EngineTiming;

///
/// a movement strategy for a free-floating, clip-free spectator.
///
/// largely intended for spectator view in games, and development.
///
// todo: this needs extensive unit testing
pub trait SpectatorMovementStrategy {
    fn move_forward(config: &EngineConfig, camera: &mut Camera, timing: &EngineTiming) {
        /* gather necessary variables */
        let forward = camera.orientation.position.column_major_z_forward();
        let position = camera.orientation.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32, );
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        camera.orientation.position.column_major_update_position(&updated);
    }

    fn move_backward(config: &EngineConfig, camera: &mut Camera, timing: &EngineTiming) {
        /* gather necessary variables */
        let forward = camera.orientation.position.column_major_z_forward();
        let position = camera.orientation.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32, );
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        camera.orientation.position.column_major_update_position(&updated);
    }

    fn move_left(config: &EngineConfig, camera: &mut Camera, timing: &EngineTiming) {
        /* gather necessary variables */
        let right = camera.orientation.position.column_major_x_right();
        let position = camera.orientation.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32, );
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        camera.orientation.position.column_major_update_position(&updated);
    }

    fn move_right(config: &EngineConfig, camera: &mut Camera, timing: &EngineTiming) {
        /* gather necessary variables */
        let right = camera.orientation.position.column_major_x_right();
        let position = camera.orientation.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32, );
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        camera.orientation.position.column_major_update_position(&updated);
    }

    ///
    /// update mouse look based on dx/dy.
    ///
    fn update_look(deltas: &Vec<MouseDelta>, config: &EngineConfig, camera: &mut Camera) {
        /* get the collective delta x and y */
        let dx = deltas.iter().map(|d| d.dx).sum::<f32>();
        let dy = deltas.iter().map(|d| d.dy).sum::<f32>();

        /* compute delta for yaw and pitch, in radians */
        let delta_yaw = dx * config.input.mouse_sensitivity * -1.0;
        let delta_pitch = dy * config.input.mouse_sensitivity * -1.0;

        /* update the yaw and pitch */
        camera.orientation.yaw.radians = camera.orientation.yaw.radians + delta_yaw;
        camera.orientation.pitch.radians = camera.orientation.pitch.radians + delta_pitch;
        camera.orientation.roll = Radians::zero();
        camera.orientation.pitch_delta.radians = delta_pitch;
        camera.orientation.yaw_delta.radians = delta_yaw;
        camera.orientation.roll_delta = Radians::zero();

        /* clamp to prevent flip */
        if camera.orientation.pitch.radians > 89.0 { camera.orientation.pitch.radians = 89.0; }
        if camera.orientation.pitch.radians < -89.0 { camera.orientation.pitch.radians = -89.0; }

        /* finally, update orientation */
        let rotation = from_pitch_yaw_roll(
            &camera.orientation.pitch_delta,
            &camera.orientation.yaw_delta,
            &camera.orientation.roll_delta,
        );
        camera.orientation.position = rotate(&camera.orientation.position, &rotation);
    }
}
