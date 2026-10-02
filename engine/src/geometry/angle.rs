use crate::geometry::angle::degrees::Degrees;
use crate::geometry::angle::radians::Radians;

pub mod degrees;
pub mod radians;
mod f32;
mod f64;

pub trait ToDegrees {
    fn to_degrees(&self) -> Degrees;
}

pub trait ToRadians {
    fn to_radians(&self) -> Radians;
}
