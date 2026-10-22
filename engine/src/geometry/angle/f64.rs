use crate::geometry::angle::radians::Radians;
use crate::geometry::angle::{ToDegrees, ToRadians};
use crate::geometry::angle::degrees::Degrees;

impl ToRadians for f64 {
    fn to_radians(&self) -> Radians {
        Radians { radians: *self as f32 }
    }
}

impl ToDegrees for f64 {
    fn to_degrees(&self) -> Degrees {
        Degrees { degrees: *self as f32 }
    }
}
