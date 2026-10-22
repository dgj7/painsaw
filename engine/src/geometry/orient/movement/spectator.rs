use crate::config::EngineConfig;
use crate::geometry::angle::degrees::Degrees;
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
    fn move_forward(&self, config: &EngineConfig, camera: &mut Camera, timing: &EngineTiming) {
        /* gather necessary variables */
        let forward = camera.orientation.transform.column_major_z_forward();
        let position = camera.orientation.transform.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32, );
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        camera.orientation.transform.column_major_update_position(&updated);
    }

    fn move_backward(&self, config: &EngineConfig, camera: &mut Camera, timing: &EngineTiming) {
        /* gather necessary variables */
        let forward = camera.orientation.transform.column_major_z_forward();
        let position = camera.orientation.transform.column_major_position();

        /* compute change (forward * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&forward, config.movement.forward_speed), timing.delta_time as f32, );
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        camera.orientation.transform.column_major_update_position(&updated);
    }

    fn move_left(&self, config: &EngineConfig, camera: &mut Camera, timing: &EngineTiming) {
        /* gather necessary variables */
        let right = camera.orientation.transform.column_major_x_right();
        let position = camera.orientation.transform.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32, );
        let updated = Vertex3D::new_subtract(&position, &change);

        /* update the orientation matrix */
        camera.orientation.transform.column_major_update_position(&updated);
    }

    fn move_right(&self, config: &EngineConfig, camera: &mut Camera, timing: &EngineTiming) {
        /* gather necessary variables */
        let right = camera.orientation.transform.column_major_x_right();
        let position = camera.orientation.transform.column_major_position();

        /* compute change (right * speed * delta_time), then update position */
        let change = Vertex3D::new_mult_scalar(&Vertex3D::new_mult_scalar(&right, config.movement.forward_speed), timing.delta_time as f32, );
        let updated = Vertex3D::new_add(&position, &change);

        /* update the orientation matrix */
        camera.orientation.transform.column_major_update_position(&updated);
    }

    ///
    /// update mouse look based on dx/dy.
    ///
    fn update_look(&self, deltas: &Vec<MouseDelta>, config: &EngineConfig, camera: &mut Camera) {
        /* preserve position before rebuilding/rotating orientation */
        let position = camera.orientation.transform.column_major_position();

        /* get the collective delta x and y */
        let dx = deltas.iter().map(|d| d.dx).sum::<f32>();
        let dy = deltas.iter().map(|d| d.dy).sum::<f32>();

        /* calculate the change in pitch and yaw based on updated mouse position */
        let delta_yaw_degrees = dx * config.input.mouse_sensitivity * -1.0;
        let delta_pitch_degrees = dy * config.input.mouse_sensitivity * -1.0;

        /* update the yaw and pitch */
        camera.orientation.yaw.degrees = camera.orientation.yaw.degrees + delta_yaw_degrees;
        camera.orientation.pitch.degrees = camera.orientation.pitch.degrees + delta_pitch_degrees;
        camera.orientation.roll = Degrees::zero();
        camera.orientation.pitch_delta.degrees = delta_pitch_degrees;
        camera.orientation.yaw_delta.degrees = delta_yaw_degrees;
        camera.orientation.roll_delta = Degrees::zero();

        /* clamp to prevent flip */
        if camera.orientation.pitch.degrees > 89.0 { camera.orientation.pitch.degrees = 89.0; }
        if camera.orientation.pitch.degrees < -89.0 { camera.orientation.pitch.degrees = -89.0; }

        /* rotate the world/camera transform based on pitch/yaw/roll */
        let rotation = from_pitch_yaw_roll(&camera.orientation.pitch_delta, &camera.orientation.yaw_delta, &camera.orientation.roll_delta);
        camera.orientation.transform = rotate(&camera.orientation.transform, &rotation);
        camera.orientation.transform.column_major_update_position(&position);

        /* update the forward/right vectors */
        let pitch = camera.orientation.pitch.degrees.to_radians();
        let yaw = camera.orientation.yaw.degrees.to_radians();
        let (sin_pitch, cos_pitch) = pitch.sin_cos();
        let (sin_yaw, cos_yaw) = yaw.sin_cos();
        let right = Vertex3D::new(cos_yaw, 0.0, -sin_yaw);
        let forward = Vertex3D::new(sin_yaw * cos_pitch, -sin_pitch, cos_yaw * cos_pitch);
        camera.orientation.transform.column_major_update_right(&right);
        camera.orientation.transform.column_major_update_forward(&forward);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::input_config::InputConfig;
    use crate::config::move_config::MoveConfig;
    use crate::config::renderer_config::RendererConfig;
    use crate::config::window_config::WindowConfig;
    use crate::input::screen::ScreenState;

    struct TestGame {}
    impl SpectatorMovementStrategy for TestGame{}

    fn prepare() -> (EngineConfig, Camera, EngineTiming) {
        let rc = RendererConfig::default();
        let config = EngineConfig::new(WindowConfig::default(), rc, InputConfig::default(), MoveConfig::default());
        let camera = Camera::new(ScreenState::new());
        let mut timing = EngineTiming::new(&config.renderer);

        timing.delta_time = 0.1;

        (config, camera, timing)
    }

    mod forward {
        use crate::geometry::orient::movement::spectator::tests::{prepare, TestGame};
        use crate::geometry::orient::movement::spectator::SpectatorMovementStrategy;
        use crate::geometry::primitive::v3d::Vertex3D;

        #[test]
        fn test_no_changes() {
            let game = TestGame {};
            let (config, mut camera, timing) = prepare();

            assert_eq!(Vertex3D::new(0.0, 0.0, 0.0), camera.orientation.transform.column_major_position());
            assert_eq!(Vertex3D::new(1.0, 0.0, 0.0), camera.orientation.transform.column_major_x_right());
            assert_eq!(Vertex3D::new(0.0, 1.0, 0.0), camera.orientation.transform.column_major_y_up());
            assert_eq!(Vertex3D::new(0.0, 0.0, 1.0), camera.orientation.transform.column_major_z_forward());
            assert_eq!(0.0, camera.orientation.pitch.degrees);
            assert_eq!(0.0, camera.orientation.roll.degrees);
            assert_eq!(0.0, camera.orientation.yaw.degrees);
            assert_eq!(0.0, camera.orientation.pitch_delta.degrees);
            assert_eq!(0.0, camera.orientation.roll_delta.degrees);
            assert_eq!(0.0, camera.orientation.yaw_delta.degrees);

            game.move_forward(&config, &mut camera, &timing);

            assert_eq!(Vertex3D::new(0.0, 0.0, -0.5), camera.orientation.transform.column_major_position());
            assert_eq!(Vertex3D::new(1.0, 0.0, 0.0), camera.orientation.transform.column_major_x_right());
            assert_eq!(Vertex3D::new(0.0, 1.0, 0.0), camera.orientation.transform.column_major_y_up());
            assert_eq!(Vertex3D::new(0.0, 0.0, 1.0), camera.orientation.transform.column_major_z_forward());
            assert_eq!(0.0, camera.orientation.pitch.degrees);
            assert_eq!(0.0, camera.orientation.roll.degrees);
            assert_eq!(0.0, camera.orientation.yaw.degrees);
            assert_eq!(0.0, camera.orientation.pitch_delta.degrees);
            assert_eq!(0.0, camera.orientation.roll_delta.degrees);
            assert_eq!(0.0, camera.orientation.yaw_delta.degrees);
        }
    }

    mod backward {
        use crate::geometry::orient::movement::spectator::tests::{prepare, TestGame};
        use crate::geometry::orient::movement::spectator::SpectatorMovementStrategy;
        use crate::geometry::primitive::v3d::Vertex3D;

        #[test]
        fn test_no_changes() {
            let game = TestGame {};
            let (config, mut camera, timing) = prepare();

            assert_eq!(Vertex3D::new(0.0, 0.0, 0.0), camera.orientation.transform.column_major_position());
            assert_eq!(Vertex3D::new(1.0, 0.0, 0.0), camera.orientation.transform.column_major_x_right());
            assert_eq!(Vertex3D::new(0.0, 1.0, 0.0), camera.orientation.transform.column_major_y_up());
            assert_eq!(Vertex3D::new(0.0, 0.0, 1.0), camera.orientation.transform.column_major_z_forward());
            assert_eq!(0.0, camera.orientation.pitch.degrees);
            assert_eq!(0.0, camera.orientation.roll.degrees);
            assert_eq!(0.0, camera.orientation.yaw.degrees);
            assert_eq!(0.0, camera.orientation.pitch_delta.degrees);
            assert_eq!(0.0, camera.orientation.roll_delta.degrees);
            assert_eq!(0.0, camera.orientation.yaw_delta.degrees);

            game.move_backward(&config, &mut camera, &timing);

            assert_eq!(Vertex3D::new(0.0, 0.0, 0.5), camera.orientation.transform.column_major_position());
            assert_eq!(Vertex3D::new(1.0, 0.0, 0.0), camera.orientation.transform.column_major_x_right());
            assert_eq!(Vertex3D::new(0.0, 1.0, 0.0), camera.orientation.transform.column_major_y_up());
            assert_eq!(Vertex3D::new(0.0, 0.0, 1.0), camera.orientation.transform.column_major_z_forward());
            assert_eq!(0.0, camera.orientation.pitch.degrees);
            assert_eq!(0.0, camera.orientation.roll.degrees);
            assert_eq!(0.0, camera.orientation.yaw.degrees);
            assert_eq!(0.0, camera.orientation.pitch_delta.degrees);
            assert_eq!(0.0, camera.orientation.roll_delta.degrees);
            assert_eq!(0.0, camera.orientation.yaw_delta.degrees);
        }
    }

    mod left {
        use crate::geometry::orient::movement::spectator::tests::{prepare, TestGame};
        use crate::geometry::orient::movement::spectator::SpectatorMovementStrategy;
        use crate::geometry::primitive::v3d::Vertex3D;

        #[test]
        fn test_no_changes() {
            let game = TestGame {};
            let (config, mut camera, timing) = prepare();

            assert_eq!(Vertex3D::new(0.0, 0.0, 0.0), camera.orientation.transform.column_major_position());
            assert_eq!(Vertex3D::new(1.0, 0.0, 0.0), camera.orientation.transform.column_major_x_right());
            assert_eq!(Vertex3D::new(0.0, 1.0, 0.0), camera.orientation.transform.column_major_y_up());
            assert_eq!(Vertex3D::new(0.0, 0.0, 1.0), camera.orientation.transform.column_major_z_forward());
            assert_eq!(0.0, camera.orientation.pitch.degrees);
            assert_eq!(0.0, camera.orientation.roll.degrees);
            assert_eq!(0.0, camera.orientation.yaw.degrees);
            assert_eq!(0.0, camera.orientation.pitch_delta.degrees);
            assert_eq!(0.0, camera.orientation.roll_delta.degrees);
            assert_eq!(0.0, camera.orientation.yaw_delta.degrees);

            game.move_left(&config, &mut camera, &timing);

            assert_eq!(Vertex3D::new(-0.5, 0.0, 0.0), camera.orientation.transform.column_major_position());
            assert_eq!(Vertex3D::new(1.0, 0.0, 0.0), camera.orientation.transform.column_major_x_right());
            assert_eq!(Vertex3D::new(0.0, 1.0, 0.0), camera.orientation.transform.column_major_y_up());
            assert_eq!(Vertex3D::new(0.0, 0.0, 1.0), camera.orientation.transform.column_major_z_forward());
            assert_eq!(0.0, camera.orientation.pitch.degrees);
            assert_eq!(0.0, camera.orientation.roll.degrees);
            assert_eq!(0.0, camera.orientation.yaw.degrees);
            assert_eq!(0.0, camera.orientation.pitch_delta.degrees);
            assert_eq!(0.0, camera.orientation.roll_delta.degrees);
            assert_eq!(0.0, camera.orientation.yaw_delta.degrees);
        }
    }

    mod right {
        use crate::geometry::orient::movement::spectator::tests::{prepare, TestGame};
        use crate::geometry::orient::movement::spectator::SpectatorMovementStrategy;
        use crate::geometry::primitive::v3d::Vertex3D;

        #[test]
        fn test_no_changes() {
            let game = TestGame {};
            let (config, mut camera, timing) = prepare();

            assert_eq!(Vertex3D::new(0.0, 0.0, 0.0), camera.orientation.transform.column_major_position());
            assert_eq!(Vertex3D::new(1.0, 0.0, 0.0), camera.orientation.transform.column_major_x_right());
            assert_eq!(Vertex3D::new(0.0, 1.0, 0.0), camera.orientation.transform.column_major_y_up());
            assert_eq!(Vertex3D::new(0.0, 0.0, 1.0), camera.orientation.transform.column_major_z_forward());
            assert_eq!(0.0, camera.orientation.pitch.degrees);
            assert_eq!(0.0, camera.orientation.roll.degrees);
            assert_eq!(0.0, camera.orientation.yaw.degrees);
            assert_eq!(0.0, camera.orientation.pitch_delta.degrees);
            assert_eq!(0.0, camera.orientation.roll_delta.degrees);
            assert_eq!(0.0, camera.orientation.yaw_delta.degrees);

            game.move_right(&config, &mut camera, &timing);

            assert_eq!(Vertex3D::new(0.5, 0.0, 0.0), camera.orientation.transform.column_major_position());
            assert_eq!(Vertex3D::new(1.0, 0.0, 0.0), camera.orientation.transform.column_major_x_right());
            assert_eq!(Vertex3D::new(0.0, 1.0, 0.0), camera.orientation.transform.column_major_y_up());
            assert_eq!(Vertex3D::new(0.0, 0.0, 1.0), camera.orientation.transform.column_major_z_forward());
            assert_eq!(0.0, camera.orientation.pitch.degrees);
            assert_eq!(0.0, camera.orientation.roll.degrees);
            assert_eq!(0.0, camera.orientation.yaw.degrees);
            assert_eq!(0.0, camera.orientation.pitch_delta.degrees);
            assert_eq!(0.0, camera.orientation.roll_delta.degrees);
            assert_eq!(0.0, camera.orientation.yaw_delta.degrees);
        }
    }
}
