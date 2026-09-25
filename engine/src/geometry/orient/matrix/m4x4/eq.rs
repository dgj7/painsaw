use crate::geometry::orient::matrix::m4x4::Matrix4x4;

///
/// implementation of partial-eq for m4x4.
///
impl PartialEq<Self> for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.r1c1 == other.r1c1
            && self.r2c1 == other.r2c1
            && self.r3c1 == other.r3c1
            && self.r4c1 == other.r4c1
            && self.r1c2 == other.r1c2
            && self.r2c2 == other.r2c2
            && self.r3c2 == other.r3c2
            && self.r4c2 == other.r4c2
            && self.r1c3 == other.r1c3
            && self.r2c3 == other.r2c3
            && self.r3c3 == other.r3c3
            && self.r4c3 == other.r4c3
            && self.r1c4 == other.r1c4
            && self.r2c4 == other.r2c4
            && self.r3c4 == other.r3c4
            && self.r4c4 == other.r4c4
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

        b.r2c3 = 3.0;

        assert_ne!(a, b);
    }
}
