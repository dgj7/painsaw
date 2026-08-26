use crate::geometry::primitive::v3d::Vertex3D;

///
/// determine if two vectors are equal.
///
impl Vertex3D {
    pub fn is_equal(&self, other: &Vertex3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

///
/// test [Vector3D::is_equal()].
///
#[cfg(test)]
mod vtx3d_test_equal {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test_equal() {
        let a = Vertex3D::new(1.0, 2.0, 3.0);
        let b = Vertex3D::new(1.0, 2.0, 3.0);

        assert_eq!(true, a.is_equal(&b));
    }

    #[test]
    fn test_not_equal() {
        let a = Vertex3D::new(1.0, 2.0, 3.0);
        let b = Vertex3D::new(1.0, 2.0, 4.0);

        assert_eq!(false, a.is_equal(&b));
    }
}
