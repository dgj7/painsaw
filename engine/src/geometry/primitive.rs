pub mod prim2d;
pub mod prim3d;
pub mod v2d;
pub mod v3d;

///
/// simple shape types.
///
pub enum PrimitiveType {
    Point { point_size: f32 },
    Line { thickness: f32 },
    LineStrip { thickness: f32 },
    Quad {},
}
