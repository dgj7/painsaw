use crate::config::EngineConfig;
use crate::geometry::primitive::v3d::{magnitude, Vertex3D};
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
    fn move_forward(camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
        /* gather necessary variables */
        let forward = camera.orientation.position.column_major_z_forward();
        let position = camera.orientation.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32);
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        camera.orientation.position.column_major_update_position(&updated);
    }

    fn move_backward(camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
        /* gather necessary variables */
        let forward = camera.orientation.position.column_major_z_forward();
        let position = camera.orientation.position.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32);
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        camera.orientation.position.column_major_update_position(&updated);
    }

    fn move_left(camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
        /* gather necessary variables */
        let right = camera.orientation.position.column_major_x_right();
        let position = camera.orientation.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32);
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        camera.orientation.position.column_major_update_position(&updated);
    }

    fn move_right(camera: &mut Camera, config: &EngineConfig, timing: &EngineTiming) {
        /* gather necessary variables */
        let right = camera.orientation.position.column_major_x_right();
        let position = camera.orientation.position.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32);
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        camera.orientation.position.column_major_update_position(&updated);
    }

    ///
    /// update mouse look based on dx/dy.
    ///
    fn update_look(
        deltas: &Vec<MouseDelta>,
        camera: &mut Camera,
        config: &EngineConfig,
    ) {
        /* get the collective delta x and y */
        let dx = deltas.iter().map(|d| d.dx).sum::<f32>();
        let dy = deltas.iter().map(|d| d.dy).sum::<f32>();

        /* compute delta for yaw and pitch, in radians */
        let delta_yaw = dx * config.input.mouse_sensitivity * -1.0;
        let delta_pitch = dy * config.input.mouse_sensitivity * -1.0;

        /* update the yaw and pitch */
        camera.orientation.yaw = camera.orientation.yaw + delta_yaw;
        camera.orientation.pitch = camera.orientation.pitch + delta_pitch;

        /* clamp to prevent flip */
        if camera.orientation.pitch > 89.0 { camera.orientation.pitch = 89.0; }
        if camera.orientation.pitch < -89.0 { camera.orientation.pitch = -89.0; }
        
        /* finally, update orientation */
        //Self::update_orientation(camera);
    }

    ///
    /// update orientation based on previously updated yaw/pitch (radians).
    ///
    // todo: move this directly into the update_look() function; needs associated unit testing as well
    fn update_orientation(
        camera: &mut Camera
    ) {
        /* calc new unit forward vector; forward changes depending on pitch/yaw; presuming default is 0,0,-1 looking down -z */
        let cos_pitch = camera.orientation.pitch.cos();
        let sin_pitch = camera.orientation.pitch.sin();
        let cos_yaw = camera.orientation.yaw.cos();
        let sin_yaw = camera.orientation.yaw.sin();
        let fx = cos_pitch * sin_yaw;
        let fy = sin_pitch;
        let fz = -cos_pitch * cos_yaw;

        /* assemble forward from new fx,fy,fz coord */
        let forward = Vertex3D::new(fx, fy, fz);
        if magnitude(&forward) > 0.0 {
            camera.orientation.position.column_major_update_forward(&forward);
        }

        /* right = cross_product(forward, up), where up defaults to 0,1,0 */
        //let up = camera.orientation.position.column_major_y_up();
        let up = Vertex3D::new(0.0, 1.0, 0.0);
        let right = Vertex3D::new_cross_product(&forward, &up);
        if magnitude(&right) > 0.0 {
            camera.orientation.position.column_major_update_right(&right);
        }

        /* up = cross_product(forward, right) */
        let up = Vertex3D::new_cross_product(&forward, &right);
        if magnitude(&up) > 0.0 {
            camera.orientation.position.column_major_update_up(&up);
        }

        /* normalize */
        camera.orientation.position.normalize();
    }
}
