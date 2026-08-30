use crate::geometry::primitive::v3d::Vertex3D;

///
/// subtracting vertexes/vectors.
///
impl Vertex3D {
    pub fn subtract(&mut self, subtrahend: &Vertex3D) {
        self.x = self.x - subtrahend.x;
        self.y = self.y - subtrahend.y;
        self.z = self.z - subtrahend.z;
    }

    pub fn new_subtract(minuend: &Vertex3D, subtrahend: &Vertex3D) -> Vertex3D {
        Vertex3D {
            x: minuend.x - subtrahend.x,
            y: minuend.y - subtrahend.y,
            z: minuend.z - subtrahend.z,
        }
    }
}

///
/// test [Vertex3D::new_subtract()].
/// this will automatically test [Vertex3D::subtract()].
#[cfg(test)]
mod test_vtx3d_subtract {
    use crate::geometry::primitive::v3d::magnitude::magnitude;
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test_positive_addend() {
        let left = Vertex3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let right = Vertex3D {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };

        let result = Vertex3D::new_subtract(&left, &right);
        assert_eq!(-2.0, result.x);
        assert_eq!(0.0, result.y);
        assert_eq!(2.0, result.z);
        assert_eq!(2.828427, magnitude(&result));
    }

    #[test]
    fn test_negative_addend() {
        let left = Vertex3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let right = Vertex3D {
            x: -3.0,
            y: -2.0,
            z: -1.0,
        };

        let result = Vertex3D::new_subtract(&left, &right);
        assert_eq!(4.0, result.x);
        assert_eq!(4.0, result.y);
        assert_eq!(4.0, result.z);
        assert_eq!(6.928203, magnitude(&result));
    }
}
