use crate::geometry::primitive::v3d::Vertex3D;

///
/// calculate the scale for the given axis.
///
/// presumes the axis is not normalized.
///
pub fn scale(axis: Vertex3D) -> f32 {
    ((axis.x * axis.x) + (axis.y * axis.y) + (axis.z * axis.z)).sqrt()
}
