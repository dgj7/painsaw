use crate::geometry::angle::radians::Radians;
use std::f32::consts::PI;

pub struct Degrees {
    pub value: f32,
}

impl Degrees {
    pub fn new(value: f32) -> Degrees {
        Degrees { value }
    }
    
    pub fn to_radians(&self) -> Radians {
        Radians { value: degrees_to_radians(self.value) }
    }
}

///
/// convert degrees to radians.
///
fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
