use crate::geometry::primitive::v3d::Vertex3D;

///
/// functions for multiplying by a scalar.
///
/// multiplying a 3d vector by a scalar scales it's distance from the origin.
///
/// it stretches or shrinks the vector, depending on sign of the multiplier; this modifies the magnitude.
///
impl Vertex3D {
    ///
    /// multiply mutable self by a multiplier.
    ///
    pub fn mult_scalar(&mut self, multiplier: f32) {
        self.x = self.x * multiplier;
        self.y = self.y * multiplier;
        self.z = self.z * multiplier;
    }

    ///
    /// clone the multiplicand, and multiply it by a scalar.
    ///
    pub fn new_mult_scalar(multiplicand: &Vertex3D, multiplier: f32) -> Vertex3D {
        let mut clone = Vertex3D {
            x: multiplicand.x,
            y: multiplicand.y,
            z: multiplicand.z,
        };
        clone.mult_scalar(multiplier);
        clone
    }
}

///
/// test [Vertex3D::new_mult_scalar()].
/// that will automatically test [Vertex3D::mult_scalar()].
///
#[cfg(test)]
mod test_vertex3d_mult_scalar {
    use crate::geometry::primitive::v3d::{magnitude, Vertex3D};

    #[test]
    fn test_positive_multiplier() {
        let input = Vertex3D {
            x: 1.0,
            y: 3.0,
            z: 5.0,
        };
        assert_eq!(5.91608, magnitude(&input));

        let mut result = Vertex3D::new_mult_scalar(&input, 2.0);
        assert_eq!(2.0, result.x);
        assert_eq!(6.0, result.y);
        assert_eq!(10.0, result.z);
        assert_eq!(11.83216, magnitude(&result));

        result.normalize();
        assert_eq!(0.16903085, result.x);
        assert_eq!(0.50709254, result.y);
        assert_eq!(0.8451542, result.z);
        assert_eq!(0.99999994, magnitude(&result));

        result.negate();
        assert_eq!(-0.16903085, result.x);
        assert_eq!(-0.50709254, result.y);
        assert_eq!(-0.8451542, result.z);
        assert_eq!(0.99999994, magnitude(&result));
    }

    #[test]
    fn test_negative_multiplier() {
        let input = Vertex3D {
            x: 2.0,
            y: -4.0,
            z: 7.0,
        };
        assert_eq!(8.306623, magnitude(&input));

        let mut result = Vertex3D::new_mult_scalar(&input, -3.0);
        assert_eq!(-6.0, result.x);
        assert_eq!(12.0, result.y);
        assert_eq!(-21.0, result.z);
        assert_eq!(24.919872, magnitude(&result));

        result.normalize();
        assert_eq!(-0.24077168, result.x);
        assert_eq!(0.48154336, result.y);
        assert_eq!(-0.8427009, result.z);
        assert_eq!(0.9999999, magnitude(&result));

        result.negate();
        assert_eq!(0.24077168, result.x);
        assert_eq!(-0.48154336, result.y);
        assert_eq!(0.8427009, result.z);
        assert_eq!(0.9999999, magnitude(&result));
    }
}
