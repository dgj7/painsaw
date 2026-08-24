use crate::geometry::primitive::v3d::Vertex3D;

///
/// simple addition of two vertexes.
///
/// represents combined or net forces.  if two people are pushing an object
/// in different directions, adding the vectors shows their combined force.
///
impl Vertex3D {
    pub fn add(&mut self, addend: &Vertex3D) {
        self.x = self.x + addend.x;
        self.y = self.y + addend.y;
        self.z = self.z + addend.z;
    }

    pub fn new_add(left_addend: &Vertex3D, right_addend: &Vertex3D) -> Vertex3D {
        let mut result = Vertex3D {
            x: left_addend.x,
            y: left_addend.y,
            z: left_addend.z,
        };
        result.add(right_addend);
        result
    }
}

///
/// test [Vertex3D::new_add()].
/// this will automatically test [Vertex3D::add()].
#[cfg(test)]
mod test_vtx3d_add {
    use crate::geometry::primitive::v3d::magnitude::magnitude;
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test_positive_addend() {
        let left = Vertex3D{x: 1.0, y: 2.0, z: 3.0};
        let right = Vertex3D{x: 3.0, y: 2.0, z: 1.0};

        let result = Vertex3D::new_add(&left, &right);
        assert_eq!(4.0, result.x);
        assert_eq!(4.0, result.y);
        assert_eq!(4.0, result.z);
        assert_eq!(6.928203, magnitude(&result));
    }

    #[test]
    fn test_negative_addend() {
        let left = Vertex3D{x: 1.0, y: 2.0, z: 3.0};
        let right = Vertex3D{x: -3.0, y: -2.0, z: -1.0};

        let result = Vertex3D::new_add(&left, &right);
        assert_eq!(-2.0, result.x);
        assert_eq!(0.0, result.y);
        assert_eq!(2.0, result.z);
        assert_eq!(2.828427, magnitude(&result));
    }
}
