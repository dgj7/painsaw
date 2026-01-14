use std::f64::consts::PI;
use num_traits::Float;

pub mod vector;
pub mod line;
pub mod dim;
pub mod matrix;
pub mod scene;

static C_2_PI: f64 = PI * 2.0;
static C_PI_OVER_2: f64 = PI / 2.0;
static C_1_OVER_PI: f64 = 1.0 / PI;
static C_1_OVER_2PI: f64 = 1.0 / C_2_PI;
static C_PI_OVER_180 : f64 = PI / 180.0;
static C_180_OVER_PI: f64 = 180.0 / PI;

///
/// wrap an angle in the range -pi..pi.
///
pub fn wrap_pi<F: Float>(mut theta: F) -> F {
    theta = theta + F::from(PI).unwrap();
    theta = theta - F::floor(theta * F::from(C_1_OVER_2PI).unwrap()) * F::from(C_2_PI).unwrap();
    theta = theta - F::from(PI).unwrap();
    theta
}

///
/// "safe" acos(), where x is "clamped" to the nearest value if out of range.
///
/// returns value in the range 0...pi.
///
pub fn safe_a_cos<F: Float>(x: F) -> F {
    /* if smaller, wrap back around */
    if x <= F::from(-1.0).unwrap() {
        return F::from(PI).unwrap();
    }

    /* if larger, return 0 */
    if x >= F::from(1.0).unwrap() {
        return F::from(0.0).unwrap();
    }

    /* if safe value, return normal call */
    F::acos(x)
}

pub fn degrees_to_radians<F: Float>(degrees: F) -> F {
    degrees * F::from(C_PI_OVER_180).unwrap()
}

pub fn radians_to_degrees<F: Float>(radians: F) -> F {
    radians * F::from(C_180_OVER_PI).unwrap()
}

pub fn fov_to_zoom<F: Float>(fov: F) -> F {
    F::from(1.0).unwrap() / F::tan(fov * F::from(0.5).unwrap())
}

pub fn zoom_to_fov<F: Float>(zoom: F) -> F {
    F::from(2.0).unwrap() * F::atan(F::from(1.0).unwrap() / zoom)
}
