use crate::geometry::primitive::v3d::Vertex3D;

///
/// calculate the magnitude.
///
/// represents the distance from the tail to its head.  tells the "strength" of the vector.
///
/// similar to absolute value.
///
impl Vertex3D {
    pub fn magnitude(&self) -> f32 {
        magnitude(&self)
    }
}

pub fn magnitude(p: &Vertex3D) -> f32 {
    ((p.x * p.x) + (p.y * p.y) + (p.z * p.z)).sqrt()
}

///
/// test [v3d::magnitude()].
#[cfg(test)]
mod test_vtx3d_magnitude {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test_positive() {
        let input = Vertex3D {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };

        assert_eq!(7.071068, input.magnitude());
    }

    #[test]
    fn test_negative() {
        let input = Vertex3D {
            x: -3.0,
            y: -4.0,
            z: -5.0,
        };

        assert_eq!(7.071068, input.magnitude());
    }
}
