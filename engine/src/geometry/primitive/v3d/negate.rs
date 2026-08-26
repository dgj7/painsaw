use crate::geometry::primitive::v3d::Vertex3D;

///
/// negate a vertex/vector.
///
/// produces a vector that is pointing in the opposite direction, with maintained magnitude.
///
impl Vertex3D {
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}

///
/// test [Vertex3d::negate()].
///
#[cfg(test)]
mod test_vtx3d_negate {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test_flip() {
        let mut input = Vertex3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        input.negate();
        assert_eq!(input.x, -1.0);
        assert_eq!(input.y, -1.0);
        assert_eq!(input.z, -1.0);

        input.negate();
        assert_eq!(input.x, 1.0);
        assert_eq!(input.y, 1.0);
        assert_eq!(input.z, 1.0);
    }
}
