use crate::geometry::angle::degrees::Degrees;
use crate::geometry::angle::radians::Radians;
use crate::geometry::angle::{ToDegrees, ToRadians};

impl ToRadians for f32 {
    fn to_radians(&self) -> Radians {
        Radians { radians: *self }
    }
}

impl ToDegrees for f32 {
    fn to_degrees(&self) -> Degrees {
        Degrees { degrees: *self }
    }
}
