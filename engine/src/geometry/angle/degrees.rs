use crate::geometry::angle::radians::Radians;
use std::f32::consts::PI;
use crate::geometry::angle::ToRadians;

#[derive(Clone)]
pub struct Degrees {
    pub degrees: f32,
}

impl Degrees {
    pub fn new(value: f32) -> Degrees {
        Degrees { degrees: value }
    }
    
    pub fn zero() -> Degrees {
        Degrees { degrees: 0.0 }
    }
}

impl ToRadians for Degrees {
    fn to_radians(&self) -> Radians {
        Radians { radians: degrees_to_radians(self.degrees), }
    }
}

///
/// convert degrees to radians.
///
fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
