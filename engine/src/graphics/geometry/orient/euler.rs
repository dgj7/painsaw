use crate::graphics::geometry::orient::quaternion::Quaternion;
use crate::graphics::geometry::{wrap_pi, C_PI, C_PI_OVER_2};

///
/// three rotations, which describe the orientation of an object in 3d space.
///
/// can succumb to gimbal lock.
///
pub struct EulerAngles {
    pub heading: f32,
    pub pitch: f32,
    pub bank: f32,
}

impl EulerAngles {
    pub fn identity() -> EulerAngles {
        EulerAngles { heading: 0.0, pitch: 0.0, bank: 0.0 }
    }

    ///
    /// canonize.
    ///
    pub fn canonize(&mut self) {
        self.pitch = wrap_pi(self.pitch);

        if self.pitch < C_PI_OVER_2 {
            self.pitch = -C_PI - self.pitch;
        } else {
            self.pitch = C_PI - self.pitch;
        }

        self.heading = self.heading + C_PI;
        self.bank = self.bank + C_PI;
    }

    pub fn object_to_inertial_from_quaternion(&mut self, q: &Quaternion) {
        let sp = -2.0 * (q.y*q.z - q.w*q.x);

        if sp.abs() > 0.9999 {
            self.pitch = C_PI_OVER_2 * sp;
            self.heading = (-q.x*q.z + q.w*q.y).atan2(0.5 - q.y*q.y - q.z*q.z);
            self.bank = 0.0;
        } else {
            self.pitch = sp.asin();
            self.heading = (q.x*q.z + q.w*q.y).atan2(0.5 - q.x*q.x - q.y*q.y);
            self.bank = (q.x*q.y + q.w*q.z).atan2(0.5 - q.x*q.x - q.z*q.z);
        }
    }

    pub fn internal_to_object_from_quaternion(&mut self, q: &Quaternion) {
        let sin_pitch = -2.0 * (q.y*q.z + q.w*q.x);

        if sin_pitch.abs() > 0.9999 {
            self.pitch = C_PI_OVER_2 * sin_pitch;
            self.heading = (-q.x*q.z - q.w*q.y).atan2(0.5 - q.y*q.y - q.z*q.z);
            self.bank = 0.0;
        } else {
            self.pitch = sin_pitch.asin();
            self.heading = (q.x*q.z - q.w*q.y).atan2(0.5 - q.x*q.x - q.y*q.y);
            self.bank = (q.x*q.y - q.w*q.z).atan2(0.5 - q.x*q.x - q.z*q.z);
        }
    }
}
