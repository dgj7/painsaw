use num_traits::Float;

pub mod prim3d;
pub mod prim2d;
pub mod v2d;
pub mod v3d;

///
/// simple shape types.
///
pub enum PrimitiveType<F: Float> {
    Point { point_size: F },
    Line { thickness: F },
    Quad {},
}
