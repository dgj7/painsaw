use crate::geometry::orient::matrix::m4x4::Matrix4x4;

///
/// implementation of partial-eq for m4x4.
///
impl PartialEq<Self> for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.c1r1 == other.c1r1
        && self.c1r2 == other.c1r2
        && self.c1r3 == other.c1r3
        && self.c1r4 == other.c1r4
        && self.c2r1 == other.c2r1
        && self.c2r2 == other.c2r2
        && self.c2r3 == other.c2r3
        && self.c2r4 == other.c2r4
        && self.c3r1 == other.c3r1
        && self.c3r2 == other.c3r2
        && self.c3r3 == other.c3r3
        && self.c3r4 == other.c3r4
        && self.c4r1 == other.c4r1
        && self.c4r2 == other.c4r2
        && self.c4r3 == other.c4r3
        && self.c4r4 == other.c4r4
    }
}

///
/// inform the compiler that the partial-eq implementation represents a full equivalence relation.
///
impl Eq for Matrix4x4 {}

#[cfg(test)]
mod tests {
    use crate::geometry::orient::matrix::m4x4::Matrix4x4;

    #[test]
    fn test_equal_1() {
        let a = Matrix4x4::identity();
        let b = Matrix4x4::identity();

        assert_eq!(a, b);
    }

    #[test]
    fn test_not_equal_1() {
        let a = Matrix4x4::identity();
        let mut b = Matrix4x4::identity();

        b.c3r2 = 3.0;

        assert_ne!(a, b);
    }
}
