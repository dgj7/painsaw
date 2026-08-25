use crate::geometry::primitive::v3d::Vertex3D;

///
/// compute the dot product
///
/// dot product: "how much" two vectors point in the same direction.
/// positive: same direction
/// zero: perpendicular
/// negative: opposite directions
///
/// computed as:
/// absolute value of left
/// x absolute value of right
/// x cosine of the angle betwixt them
///
impl Vertex3D {

    pub fn dot_product(&self, multiplier: &Vertex3D) -> f32 {
        self.x * multiplier.x + self.y * multiplier.y + self.z * multiplier.z
    }
}

///
/// test [Vertex3D::dot_product].
///
#[cfg(test)]
mod vtx3d_test_dot_product {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test_same() {
        let left = Vertex3D {
            x: 2.0,
            y: 2.0,
            z: 1.0,
        };
        let right = Vertex3D {
            x: 6.0,
            y: 6.0,
            z: 1.0,
        };

        assert_eq!(25.0, left.dot_product(&right));
    }

    #[test]
    fn test_perpendicular() {
        let left = Vertex3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let right = Vertex3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        assert_eq!(0.0, left.dot_product(&right));
    }

    #[test]
    fn test_negative() {
        let left = Vertex3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let right = Vertex3D {
            x: -2.0,
            y: -1.0,
            z: 0.0,
        };

        assert_eq!(-4.0, left.dot_product(&right));
    }
}
