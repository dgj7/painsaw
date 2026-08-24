
// todo: unit test this


use crate::geometry::primitive::v3d::Vertex3D;

impl Vertex3D {
    // todo: add a mutable self method


    pub fn new_div_scalar(dividend: &Vertex3D, divisor: f32) -> Vertex3D {
        Vertex3D {
            x: dividend.x / divisor,
            y: dividend.y / divisor,
            z: dividend.z / divisor,
        }
    }
}
