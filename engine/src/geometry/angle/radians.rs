use crate::geometry::angle::degrees::Degrees;
use std::f32::consts::PI;

///
/// radians: unit of angular measurement.  angle superimposed on a circle where the arc
/// is the length of the radius of the circle.  a circle is about 2*pi (6.28) radians,
/// making 1 radian roughly equal to 57.3 degrees.
///
pub struct Radians {
    pub radians: f32,
}

impl Radians {
    pub fn new(value: f32) -> Radians {
        Radians { radians: value }
    }
    
    pub fn to_degrees(&self) -> Degrees {
        Degrees { degrees: radians_to_degrees(self.radians) }
    }
}

///
/// convert radians to degrees.
///
fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}
