use crate::geometry::primitive::v3d::Vertex3D;

///
/// the cross product.
///
/// presuming 2 initial vectors (a plane), the cross product is
/// a new 3rd vector representing a perpendicular to the original 2.
///
impl Vertex3D {
    pub fn new_cross_product(left: &Vertex3D, right: &Vertex3D) -> Vertex3D {
        Vertex3D {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x,
        }
    }
}

///
/// test [Vector3D::new_cross_product()].
///
#[cfg(test)]
mod test_vtx3d_cross_product {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test1() {
        let a = Vertex3D::new(1.0, 3.0, 4.0);
        let b = Vertex3D::new(2.0, 7.0, -5.0);

        let result = Vertex3D::new_cross_product(&a, &b);

        assert_eq!(-43.0, result.x);
        assert_eq!(13.0, result.y);
        assert_eq!(1.0, result.z);
    }

    #[test]
    fn test2() {
        let a = Vertex3D::new(2.0, 0.0, -1.0);
        let b = Vertex3D::new(1.0, 2.0, 3.0);

        let result = Vertex3D::new_cross_product(&a, &b);

        assert_eq!(2.0, result.x);
        assert_eq!(-7.0, result.y);
        assert_eq!(4.0, result.z);
    }

    #[test]
    fn test3() {
        let a = Vertex3D::new(1.0, 0.0, 0.0);
        let b = Vertex3D::new(0.0, 1.0, 0.0);

        let result = Vertex3D::new_cross_product(&a, &b);

        assert_eq!(0.0, result.x);
        assert_eq!(0.0, result.y);
        assert_eq!(1.0, result.z);
    }
}
