use num_traits::Float;

pub mod point;
pub mod line;
pub mod p3d;

///
/// simple shape types.
///
pub enum PrimitiveType<F: Float> {
    Point { point_size: F },
    Line { thickness: F },
}
