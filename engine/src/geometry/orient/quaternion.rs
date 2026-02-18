use crate::geometry::orient::euler::EulerAngles;
use crate::geometry::primitive::v3d::{magnitude, Vertex3D};
use crate::geometry::safe_a_cos;

///
/// 4d number for storing rotations or orientations.
///
/// consists of a scalar part (w, angle theta), and
/// three imaginary parts (point of x, y, z).
///
/// doesn't suffer from gimbal lock.
///
#[derive(Clone)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {
    pub fn rotate_about_x(&mut self, theta: f32) {
        let theta_over_2 = theta * 0.5;

        self.w = theta_over_2.cos();

        self.x = theta_over_2.sin();
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn rotate_about_y(&mut self, theta: f32) {
        let theta_over_2 = theta * 0.5;

        self.w = theta_over_2.cos();

        self.x = 0.0;
        self.y = theta_over_2.sin();
        self.z = 0.0;
    }

    pub fn rotate_about_z(&mut self, theta: f32) {
        let theta_over_2 = theta * 0.5;

        self.w = theta_over_2.cos();

        self.x = 0.0;
        self.y = 0.0;
        self.z = theta_over_2.sin();
    }

    pub fn rotate_about_axis(&mut self, axis: &Vertex3D, theta: f32) {
        let mag_minus_1 = magnitude(&axis) - 1.0;
        let fabs_mag = mag_minus_1.abs();
        assert!(fabs_mag < 0.1);

        let theta_over_2 = theta * 0.5;
        let sin_over_theta_2 = theta_over_2.sin();

        self.w = theta_over_2.cos();

        self.x = axis.x * sin_over_theta_2;
        self.y = axis.y * sin_over_theta_2;
        self.z = axis.z * sin_over_theta_2;
    }

    ///
    /// rotate from object space to inertial space.
    ///
    /// note that inertial space is 'transitional' space between object/storage
    /// space and world space (coordinates).
    ///
    pub fn rotate_object_to_inertial(&mut self, orientation: &EulerAngles) {
        let sine_pitch = orientation.pitch * 0.5;
        let cosine_pitch = orientation.pitch * 0.5;

        let sine_bank = orientation.bank * 0.5;
        let cosine_bank = orientation.bank * 0.5;

        let sine_heading = orientation.heading * 0.5;
        let cosine_heading = orientation.heading * 0.5;

        self.w =  cosine_heading * cosine_pitch * cosine_bank + sine_heading * sine_pitch * sine_bank;
        self.x =  cosine_heading * sine_pitch * cosine_bank + sine_heading * cosine_pitch * sine_bank;
        self.y = -cosine_heading * sine_pitch * sine_bank + sine_heading * cosine_pitch * cosine_bank;
        self.z = -sine_heading * sine_pitch * cosine_bank + cosine_heading * cosine_pitch * sine_bank;
    }

    pub fn rotate_inertial_to_object(&mut self, orientation: &EulerAngles) {
        let sine_pitch = orientation.pitch * 0.5;
        let cosine_pitch = orientation.pitch * 0.5;

        let sine_bank = orientation.bank * 0.5;
        let cosine_bank = orientation.bank * 0.5;

        let sine_heading = orientation.heading * 0.5;
        let cosine_heading = orientation.heading * 0.5;

        self.w =  cosine_heading * cosine_pitch * cosine_bank + sine_heading * sine_pitch * sine_bank;
        self.x = -cosine_heading * sine_pitch * cosine_bank - sine_heading * cosine_pitch * sine_bank;
        self.y =  cosine_heading * sine_pitch * sine_bank - sine_heading * cosine_bank * cosine_pitch;
        self.z =  sine_heading * sine_pitch * cosine_bank - cosine_heading * cosine_pitch * sine_bank;
    }

    pub fn cross_product(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            w: self.w* other.w - self.x* other.x - self.y* other.y - self.z* other.z,
            x: self.w* other.x + self.x* other.w + self.z* other.y - self.y* other.z,
            y: self.w* other.y + self.y* other.w + self.x* other.z - self.z* other.x,
            z: self.w* other.z + self.z* other.w + self.y* other.x - self.x* other.y,
        }
    }

    pub fn normalize(&mut self) {
        let magnitude = (self.w*self.w + self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        let one_over_mag = 1.0 / magnitude;

        self.w = self.w * one_over_mag;
        self.x = self.x * one_over_mag;
        self.y = self.y * one_over_mag;
        self.z = self.z * one_over_mag;
    }

    pub fn to_rotation_angle(&self) -> f32 {
        let theta_over_2 = safe_a_cos(self.w);
        theta_over_2 * 2.0
    }

    pub fn to_rotation_axis(&self) -> Vertex3D {
        let sin_theta_over_2_squared = 1.0 - (self.w * self.w);

        if sin_theta_over_2_squared <= 0.0 {
            return Vertex3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
        }

        let one_over_sin_theta_over_2 = 1.0 / sin_theta_over_2_squared.sqrt();
        Vertex3D {
            x: self.x * one_over_sin_theta_over_2,
            y: self.y * one_over_sin_theta_over_2,
            z: self.z * one_over_sin_theta_over_2,
        }
    }
}

impl Quaternion {
    pub fn identity() -> Quaternion {
        Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub fn dot_product(left: &Quaternion, right: &Quaternion) -> f32 {
    left.w*right.w + left.x*right.x + left.y*right.y + left.z*right.z
}

///
/// spherical linear interpolation.
///
pub fn slerp(q0: &Quaternion, q1: &Quaternion, t: f32) -> Quaternion {
    if t <= 0.0 {
        return q0.clone();
    }

    if t >= 1.0 {
        return q1.clone();
    }

    let mut cos_omega = dot_product(q0, q1);
    let mut q1w = q1.w;
    let mut q1x = q1.x;
    let mut q1y = q1.y;
    let mut q1z = q1.z;
    if cos_omega < 0.0 {
        q1w = -q1w;
        q1x = -q1x;
        q1y = -q1y;
        q1z = -q1z;
        cos_omega = -cos_omega;
    }

    assert!(cos_omega < 1.1);

    let k0;
    let k1;

    if cos_omega > 0.9999 {
        k0 = 1.0 - t;
        k1 = t;
    } else {
        let sin_omega = (1.0 - cos_omega*cos_omega).sqrt();
        let omega = sin_omega.atan2(cos_omega);
        let one_over_sin_omega = 1.0 / sin_omega;
        k0 = (1.0 - t) * omega * one_over_sin_omega;
        k1 = (t * omega) * one_over_sin_omega;
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
pub fn conjugate(other: &Quaternion) -> Quaternion {
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
pub fn pow(q: &Quaternion, exponent: f32) -> Quaternion {
    if q.w.abs() > 0.9999 {
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


