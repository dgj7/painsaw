use crate::math::threed::point_3d::Point3D;

pub struct Line3D {
    pub a: Point3D,
    pub b: Point3D,
}

impl Line3D {
    pub fn new(a: Point3D, b: Point3D) -> Line3D {
        Line3D { a, b }
    }
}
