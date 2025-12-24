use num_traits::Num;
use crate::math::threed::point_3d::Point3D;

pub struct Line3D<F: Num> {
    pub a: Point3D<F>,
    pub b: Point3D<F>,
}

impl<F: Num> Line3D<F> {
    pub fn new(a: Point3D<F>, b: Point3D<F>) -> Line3D<F> {
        Line3D { a, b }
    }
}
