use crate::geometry::quaternion::Quaternion;
use crate::geometry::{wrap_pi, C_PI_OVER_2};
use num_traits::Float;
use std::f64::consts::PI;
use num_traits::real::Real;

///
/// three rotations, which describe the orientation of an object in 3d space.
///
/// can succumb to gimbal lock.
///
pub struct EulerAngles<F: Float> {
    pub heading: F,
    pub pitch: F,
    pub bank: F,
}

impl<F: Float> EulerAngles<F> {
    pub fn identity() -> EulerAngles<F> {
        EulerAngles { heading: F::zero(), pitch: F::zero(), bank: F::zero() }
    }

    ///
    /// canonize.
    ///
    pub fn canonize(&mut self) {
        self.pitch = wrap_pi(self.pitch);

        if self.pitch < F::from(C_PI_OVER_2).unwrap() {
            self.pitch = F::from(-PI).unwrap() - self.pitch;
        } else {
            self.pitch = F::from(PI).unwrap() - self.pitch;
        }

        self.heading = self.heading + F::from(PI).unwrap();
        self.bank = self.bank + F::from(PI).unwrap();
    }

    pub fn object_to_inertial_from_quaternion(&mut self, q: &Quaternion<F>) {
        let sp = F::from(-2.0).unwrap() * (q.y*q.z - q.w*q.x);

        if sp.abs() > F::from(0.9999).unwrap() {
            self.pitch = F::from(C_PI_OVER_2).unwrap() * sp;
            self.heading = F::from(-q.x*q.z + q.w*q.y).unwrap().atan2(F::from(F::from(0.5).unwrap() - q.y*q.y - q.z*q.z).unwrap());
            self.bank = F::zero();
        } else {
            self.pitch = sp.asin();
            self.heading = F::from(q.x*q.z + q.w*q.y).unwrap().atan2(F::from(0.5).unwrap() - q.x*q.x - q.y*q.y);
            self.bank = F::from(q.x*q.y + q.w*q.z).unwrap().atan2(F::from(0.5).unwrap() - q.x*q.x - q.z*q.z);
        }
    }

    pub fn internal_to_object_from_quaternion(&mut self, q: &Quaternion<F>) {
        let sin_pitch = F::from(-2.0).unwrap() * (q.y*q.z + q.w*q.x);

        if sin_pitch.abs() > F::from(0.9999).unwrap() {
            self.pitch = F::from(C_PI_OVER_2).unwrap() * sin_pitch;
            self.heading = F::from(-q.x*q.z - q.w*q.y).unwrap().atan2(F::from(0.5).unwrap() - q.y*q.y - q.z*q.z);
            self.bank = F::zero();
        } else {
            self.pitch = sin_pitch.asin();
            self.heading = F::from(q.x*q.z - q.w*q.y).unwrap().atan2(F::from(0.5).unwrap() - q.x*q.x - q.y*q.y);
            self.bank = F::from(q.x*q.y - q.w*q.z).unwrap().atan2(F::from(0.5).unwrap() - q.x*q.x - q.z*q.z);
        }
    }
}
