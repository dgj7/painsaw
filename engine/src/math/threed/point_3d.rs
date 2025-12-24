use num_traits::Num;

pub struct Point3D<F: Num> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Num> Point3D<F> {
    pub fn new(x: F, y: F, z: F) -> Point3D<F> {
        Point3D { x, y, z }
    }

    pub fn origin() -> Point3D<F> {
        Point3D::new(F::zero(), F::zero(), F::zero())
    }
}
