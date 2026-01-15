use crate::geometry::euler::EulerAngles;
use crate::geometry::point::p3d::{magnitude, Point3D};
use crate::geometry::safe_a_cos;
use num_traits::Float;

///
/// 4d number for storing rotations or orientations.
///
/// consists of a scalar part (w, angle theta), and
/// three imaginary parts (point of x, y, z).
///
/// doesn't suffer from gimbal lock.
///
#[derive(Clone)]
pub struct Quaternion<F: Float> {
    pub w: F,
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Quaternion<F> {
    pub fn rotate_about_x(&mut self, theta: F) {
        let theta_over_2: F = theta * F::from(0.5).unwrap();

        self.w = theta_over_2.cos();

        self.x = theta_over_2.sin();
        self.y = F::zero();
        self.z = F::zero();
    }

    pub fn rotate_about_y(&mut self, theta: F) {
        let theta_over_2: F = theta * F::from(0.5).unwrap();

        self.w = theta_over_2.cos();

        self.x = F::zero();
        self.y = theta_over_2.sin();
        self.z = F::zero();
    }

    pub fn rotate_about_z(&mut self, theta: F) {
        let theta_over_2: F = theta * F::from(0.5).unwrap();

        self.w = theta_over_2.cos();

        self.x = F::zero();
        self.y = F::zero();
        self.z = theta_over_2.sin();
    }

    pub fn rotate_about_axis(&mut self, axis: &Point3D<F>, theta: F) {
        let mag_minus_1 = magnitude(&axis) - F::one();
        let fabs_mag = F::abs(mag_minus_1);
        assert!(fabs_mag < F::from(0.1).unwrap());

        let theta_over_2 = theta * F::from(0.5).unwrap();
        let sin_over_theta_2 = theta_over_2.sin();

        self.w = theta_over_2.cos();

        self.x = axis.x * sin_over_theta_2;
        self.y = axis.y * sin_over_theta_2;
        self.z = axis.z * sin_over_theta_2;
    }

    ///
    /// rotate from object space to inertial space.
    ///
    /// note that inertial space is 'transitional' space between object/model
    /// space and world space (coordinates).
    ///
    pub fn rotate_object_to_inertial(&mut self, orientation: &EulerAngles<F>) {
        let sine_pitch = F::from(orientation.pitch * F::from(0.5).unwrap()).unwrap();
        let cosine_pitch = F::from(orientation.pitch * F::from(0.5).unwrap()).unwrap();

        let sine_bank = F::from(orientation.bank * F::from(0.5).unwrap()).unwrap();
        let cosine_bank = F::from(orientation.bank * F::from(0.5).unwrap()).unwrap();

        let sine_heading = F::from(orientation.heading * F::from(0.5).unwrap()).unwrap();
        let cosine_heading = F::from(orientation.heading * F::from(0.5).unwrap()).unwrap();

        self.w =  cosine_heading * cosine_pitch * cosine_bank + sine_heading * sine_pitch * sine_bank;
        self.x =  cosine_heading * sine_pitch * cosine_bank + sine_heading * cosine_pitch * sine_bank;
        self.y = -cosine_heading * sine_pitch * sine_bank + sine_heading * cosine_pitch * cosine_bank;
        self.z = -sine_heading * sine_pitch * cosine_bank + cosine_heading * cosine_pitch * sine_bank;
    }

    pub fn rotate_inertial_to_object(&mut self, orientation: &EulerAngles<F>) {
        let sine_pitch = F::from(orientation.pitch * F::from(0.5).unwrap()).unwrap();
        let cosine_pitch = F::from(orientation.pitch * F::from(0.5).unwrap()).unwrap();

        let sine_bank = F::from(orientation.bank * F::from(0.5).unwrap()).unwrap();
        let cosine_bank = F::from(orientation.bank * F::from(0.5).unwrap()).unwrap();

        let sine_heading = F::from(orientation.heading * F::from(0.5).unwrap()).unwrap();
        let cosine_heading = F::from(orientation.heading * F::from(0.5).unwrap()).unwrap();

        self.w =  cosine_heading * cosine_pitch * cosine_bank + sine_heading * sine_pitch * sine_bank;
        self.x = -cosine_heading * sine_pitch * cosine_bank - sine_heading * cosine_pitch * sine_bank;
        self.y =  cosine_heading * sine_pitch * sine_bank - sine_heading * cosine_bank * cosine_pitch;
        self.z =  sine_heading * sine_pitch * cosine_bank - cosine_heading * cosine_pitch * sine_bank;
    }

    pub fn cross_product(&self, other: &Quaternion<F>) -> Quaternion<F> {
        Quaternion {
            w: self.w* other.w - self.x* other.x - self.y* other.y - self.z* other.z,
            x: self.w* other.x + self.x* other.w + self.z* other.y - self.y* other.z,
            y: self.w* other.y + self.y* other.w + self.x* other.z - self.z* other.x,
            z: self.w* other.z + self.z* other.w + self.y* other.x - self.x* other.y,
        }
    }

    pub fn normalize(&mut self) {
        let magnitude = F::from(self.w*self.w + self.x*self.x + self.y*self.y + self.z*self.z).unwrap().sqrt();
        let one_over_mag = F::one() / magnitude;

        self.w = self.w * one_over_mag;
        self.x = self.x * one_over_mag;
        self.y = self.y * one_over_mag;
        self.z = self.z * one_over_mag;
    }

    pub fn to_rotation_angle(&self) -> F {
        let theta_over_2 = safe_a_cos(self.w);
        F::from(theta_over_2 * F::from(2.0).unwrap()).unwrap()
    }

    pub fn to_rotation_axis(&self) -> Point3D<F> {
        let sin_theta_over_2_squared = F::from(1.0).unwrap() - (self.w * self.w);

        if sin_theta_over_2_squared <= F::from(0.0).unwrap() {
            return Point3D {
                x: F::one(),
                y: F::zero(),
                z: F::zero(),
            };
        }

        let one_over_sin_theta_over_2 = F::one() / sin_theta_over_2_squared.sqrt();
        Point3D {
            x: self.x * one_over_sin_theta_over_2,
            y: self.y * one_over_sin_theta_over_2,
            z: self.z * one_over_sin_theta_over_2,
        }
    }
}

impl<F: Float> Quaternion<F> {
    pub fn identity() -> Quaternion<F> {
        Quaternion {
            w: F::one(),
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        }
    }
}

pub fn dot_product<F: Float>(left: &Quaternion<F>, right: &Quaternion<F>) -> F {
    left.w*right.w + left.x*right.x + left.y*right.y + left.z*right.z
}

///
/// spherical linear interpolation.
///
pub fn slerp<F: Float>(q0: &Quaternion<F>, q1: &Quaternion<F>, t: F) -> Quaternion<F> {
    if t <= F::zero() {
        return q0.clone();
    }

    if t >= F::one() {
        return q1.clone();
    }

    let mut cos_omega = dot_product(q0, q1);
    let mut q1w = q1.w;
    let mut q1x = q1.x;
    let mut q1y = q1.y;
    let mut q1z = q1.z;
    if cos_omega < F::zero() {
        q1w = -q1w;
        q1x = -q1x;
        q1y = -q1y;
        q1z = -q1z;
        cos_omega = -cos_omega;
    }

    assert!(cos_omega < F::from(1.1).unwrap());

    let k0;
    let k1;

    if cos_omega > F::from(0.9999).unwrap() {
        k0 = F::one() - t;
        k1 = t;
    } else {
        let sin_omega = F::from(F::one() - cos_omega*cos_omega).unwrap().sqrt();
        let omega = sin_omega.atan2(cos_omega);
        let one_over_sin_omega = F::one() / sin_omega;
        k0 = F::from((F::one() - t) * omega).unwrap() * one_over_sin_omega;
        k1 = F::from((t * omega) * one_over_sin_omega).unwrap();
    }

    Quaternion {
        x: k0*q0.x + k1*q1x,
        y: k0*q0.y + k1*q1y,
        z: k0*q0.z + k1*q1z,
        w: k0*q0.w + k1*q1w,
    }
}

///
/// conjugate - produces quaternion with opposite rotation.
///
pub fn conjugate<F: Float>(other: &Quaternion<F>) -> Quaternion<F> {
    Quaternion {
        w: other.w,
        x: -other.x,
        y: -other.y,
        z: -other.z,
    }
}

///
/// quaternion exponentiation.
///
pub fn pow<F: Float>(q: &Quaternion<F>, exponent: F) -> Quaternion<F> {
    if F::abs(q.w) > F::from(0.9999).unwrap() {
        return q.clone();
    }

    let alpha = q.w.acos();
    let exp_alpha = alpha * exponent;
    let mult = exp_alpha.sin() / alpha.sin();

    Quaternion {
        w: exp_alpha.cos(),
        x: q.x * mult,
        y: q.y * mult,
        z: q.z * mult,
    }
}


