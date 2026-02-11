use std::f64::consts::PI;

pub mod build;
pub mod dim;
pub mod orient;
pub mod primitive;

pub(crate) static C_PI: f32 = PI as f32;
pub(crate) static C_2_PI: f32 = C_PI * 2.0;
pub(crate) static C_PI_OVER_2: f32 = C_PI / 2.0;
#[allow(unused)] // todo: remove
pub(crate) static C_1_OVER_PI: f32 = 1.0 / C_PI;
pub(crate) static C_1_OVER_2PI: f32 = 1.0 / C_2_PI;
pub(crate) static C_PI_OVER_180: f32 = C_PI / 180.0;
pub(crate) static C_180_OVER_PI: f32 = 180.0 / C_PI;

///
/// wrap an angle in the range -pi..pi.
///
pub fn wrap_pi(mut theta: f32) -> f32 {
    theta = theta + C_PI;
    theta = theta - (theta * C_1_OVER_2PI).floor() * C_2_PI;
    theta = theta - C_PI;
    theta
}

///
/// "safe" acos(), where x is "clamped" to the nearest value if out of range.
///
/// returns value in the range 0...pi.
///
pub fn safe_a_cos(x: f32) -> f32 {
    /* if smaller, wrap back around */
    if x <= -1.0 {
        return C_PI;
    }

    /* if larger, return 0 */
    if x >= 1.0 {
        return 0.0;
    }

    /* if safe value, return normal call */
    x.acos()
}

///
/// convert degrees to radians.
///
/// radians: unit of angular measurement.  angle superimposed on a circle where the arc
/// is the length of the radius of the circle.  a circle is about 2*pi (6.28) radians,
/// making 1 radian roughly equal to 57.3 degrees.
///
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * C_PI_OVER_180
}

///
/// convert radians to degrees.
///
pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * C_180_OVER_PI
}

///
/// fov (field of view) to zoom conversion.
///
/// fov: how much of the game world is visible.  measured as an angle from the camera's perspective.
///
/// lower fov (narrow angle) makes objects bigger, by zooming in.
/// higher fov (wider angle) makes objects smaller, by zooming out.
///
pub fn fov_to_zoom(fov: f32) -> f32 {
    1.0 / (fov * 0.5).tan()
}

///
/// zoom to fov (field of view) conversion.
///
pub fn zoom_to_fov(zoom: f32) -> f32 {
    2.0 * (1.0 / zoom).atan()
}
