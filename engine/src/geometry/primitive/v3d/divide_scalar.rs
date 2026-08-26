use crate::geometry::primitive::v3d::Vertex3D;

///
/// division of vectors.
///
impl Vertex3D {
    pub fn divide(&mut self, divisor: f32) {
        self.x = self.x / divisor;
        self.y = self.y / divisor;
        self.z = self.z / divisor;
    }

    pub fn new_div_scalar(dividend: &Vertex3D, divisor: f32) -> Vertex3D {
        let mut copy = Vertex3D::new(dividend.x, dividend.y, dividend.z);
        copy.divide(divisor);
        copy
    }
}

///
/// test [Vector3D::new_div_scalar()].
/// automatically tests [Vector3D::divide()].
///
#[cfg(test)]
mod test_vtx3d_divide {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test1() {
        let mut input = Vertex3D::new(6.0, 9.0, 12.0);
        input.divide(3.0);

        assert_eq!(2.0, input.x);
        assert_eq!(3.0, input.y);
        assert_eq!(4.0, input.z);
    }
}
