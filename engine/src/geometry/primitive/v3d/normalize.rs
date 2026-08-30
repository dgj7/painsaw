use crate::geometry::primitive::v3d::Vertex3D;

///
/// normalize a 3d vertex.
///
/// normalization scales the vertex/vector so that it's magnitude becomes
/// exactly 1.0, while retaining its "direction/heading" (relationship to origin).
///
impl Vertex3D {
    pub fn normalize(&mut self) {
        let magnitude_squared = self.x * self.x + self.y * self.y + self.z * self.z;
        if magnitude_squared > 0.0 {
            let one_div_mag = 1.0 / magnitude_squared.sqrt();
            self.x = self.x * one_div_mag;
            self.y = self.y * one_div_mag;
            self.z = self.z * one_div_mag;
        }
    }
}

///
/// test [Vertex3D::normalize()].
///
#[cfg(test)]
mod test_vtx3d_normalize {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test1() {
        /* create input */
        let mut input = Vertex3D {
            x: 4.0,
            y: 6.0,
            z: 9.0,
        };

        /* store the original magnitude */
        let magnitude = input.magnitude();
        assert_eq!(11.532562, magnitude);

        /*  normalize; direction is maintained; the magnitude is now 1 */
        input.normalize();
        assert_eq!(0.346844, input.x);
        assert_eq!(0.520266, input.y);
        assert_eq!(0.78039896, input.z);
        assert_eq!(1.0, input.magnitude());

        /* de-normalize by multiplying by the original magnitude; original coordinates return */
        input.mult_scalar(magnitude);
        assert_eq!(4.0, input.x);
        assert_eq!(6.0, input.y);
        assert_eq!(9.0, input.z);
    }
}
