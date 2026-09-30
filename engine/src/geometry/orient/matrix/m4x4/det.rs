use crate::geometry::orient::matrix::m4x4::Matrix4x4;

impl Matrix4x4 {
    ///
    /// calculate cofactors for this matrix.
    ///
    pub fn cofactors(&self) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
        let c11 = self.r2c2 * (self.r3c3 * self.r4c4 - self.r3c4 * self.r4c3)
            - self.r2c3 * (self.r3c2 * self.r4c4 - self.r3c4 * self.r4c2)
            + self.r2c4 * (self.r3c2 * self.r4c3 - self.r3c3 * self.r4c2);
        let c12 = -(self.r2c1 * (self.r3c3 * self.r4c4 - self.r3c4 * self.r4c3)
            - self.r2c3 * (self.r3c1 * self.r4c4 - self.r3c4 * self.r4c1)
            + self.r2c4 * (self.r3c1 * self.r4c3 - self.r3c3 * self.r4c1));
        let c13 = self.r2c1 * (self.r3c2 * self.r4c4 - self.r3c4 * self.r4c2)
            - self.r2c2 * (self.r3c1 * self.r4c4 - self.r3c4 * self.r4c1)
            + self.r2c4 * (self.r3c1 * self.r4c2 - self.r3c2 * self.r4c1);
        let c14 = -(self.r2c1 * (self.r3c2 * self.r4c3 - self.r3c3 * self.r4c2)
            - self.r2c2 * (self.r3c1 * self.r4c3 - self.r3c3 * self.r4c1)
            + self.r2c3 * (self.r3c1 * self.r4c2 - self.r3c2 * self.r4c1));
        let c21 = -(self.r1c2 * (self.r3c3 * self.r4c4 - self.r3c4 * self.r4c3)
            - self.r1c3 * (self.r3c2 * self.r4c4 - self.r3c4 * self.r4c2)
            + self.r1c4 * (self.r3c2 * self.r4c3 - self.r3c3 * self.r4c2));
        let c22 = self.r1c1 * (self.r3c3 * self.r4c4 - self.r3c4 * self.r4c3)
            - self.r1c3 * (self.r3c1 * self.r4c4 - self.r3c4 * self.r4c1)
            + self.r1c4 * (self.r3c1 * self.r4c3 - self.r3c3 * self.r4c1);
        let c23 = -(self.r1c1 * (self.r3c2 * self.r4c4 - self.r3c4 * self.r4c2)
            - self.r1c2 * (self.r3c1 * self.r4c4 - self.r3c4 * self.r4c1)
            + self.r1c4 * (self.r3c1 * self.r4c2 - self.r3c2 * self.r4c1));
        let c24 = self.r1c1 * (self.r3c2 * self.r4c3 - self.r3c3 * self.r4c2)
            - self.r1c2 * (self.r3c1 * self.r4c3 - self.r3c3 * self.r4c1)
            + self.r1c3 * (self.r3c1 * self.r4c2 - self.r3c2 * self.r4c1);
        let c31 = self.r1c2 * (self.r2c3 * self.r4c4 - self.r2c4 * self.r4c3)
            - self.r1c3 * (self.r2c2 * self.r4c4 - self.r2c4 * self.r4c2)
            + self.r1c4 * (self.r2c2 * self.r4c3 - self.r2c3 * self.r4c2);
        let c32 = -(self.r1c1 * (self.r2c3 * self.r4c4 - self.r2c4 * self.r4c3)
            - self.r1c3 * (self.r2c1 * self.r4c4 - self.r2c4 * self.r4c1)
            + self.r1c4 * (self.r2c1 * self.r4c3 - self.r2c3 * self.r4c1));
        let c33 = self.r1c1 * (self.r2c2 * self.r4c4 - self.r2c4 * self.r4c2)
            - self.r1c2 * (self.r2c1 * self.r4c4 - self.r2c4 * self.r4c1)
            + self.r1c4 * (self.r2c1 * self.r4c2 - self.r2c2 * self.r4c1);
        let c34 = -(self.r1c1 * (self.r2c2 * self.r4c3 - self.r2c3 * self.r4c2)
            - self.r1c2 * (self.r2c1 * self.r4c3 - self.r2c3 * self.r4c1)
            + self.r1c3 * (self.r2c1 * self.r4c2 - self.r2c2 * self.r4c1));
        let c41 = -(self.r1c2 * (self.r2c3 * self.r3c4 - self.r2c4 * self.r3c3)
            - self.r1c3 * (self.r2c2 * self.r3c4 - self.r2c4 * self.r3c2)
            + self.r1c4 * (self.r2c2 * self.r3c3 - self.r2c3 * self.r3c2));
        let c42 = self.r1c1 * (self.r2c3 * self.r3c4 - self.r2c4 * self.r3c3)
            - self.r1c3 * (self.r2c1 * self.r3c4 - self.r2c4 * self.r3c1)
            + self.r1c4 * (self.r2c1 * self.r3c3 - self.r2c3 * self.r3c1);
        let c43 = -(self.r1c1 * (self.r2c2 * self.r3c4 - self.r2c4 * self.r3c2)
            - self.r1c2 * (self.r2c1 * self.r3c4 - self.r2c4 * self.r3c1)
            + self.r1c4 * (self.r2c1 * self.r3c2 - self.r2c2 * self.r3c1));
        let c44 = self.r1c1 * (self.r2c2 * self.r3c3 - self.r2c3 * self.r3c2)
            - self.r1c2 * (self.r2c1 * self.r3c3 - self.r2c3 * self.r3c1)
            + self.r1c3 * (self.r2c1 * self.r3c2 - self.r2c2 * self.r3c1);
        (c11,c12,c13,c14,c21,c22,c23,c24,c31,c32,c33,c34,c41,c42,c43,c44)
    }

    ///
    /// calculate the determinant of this matrix.
    ///
    pub fn determinant(&self) -> f32 {
        /* calculate cofactors */
        let (c11,c12,c13,c14,_,_,_,_,_,_,_,_,_,_,_,_) = self.cofactors();

        /* calculate determinant; consists of the first column and its cofactors */
        self.r1c1 * c11 + self.r1c2 * c12 + self.r1c3 * c13 + self.r1c4 * c14
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::orient::matrix::m4x4::Matrix4x4;

    #[test]
    fn test_identity() {
        let m = Matrix4x4::identity();
        assert_eq!(1.0, m.determinant());
    }

    #[test]
    fn test_diagonal_scaled() {
        let mut m = Matrix4x4::identity();
        m.r1c1 = 2.0;
        m.r2c2 = 3.0;
        m.r3c3 = 4.0;
        m.r4c4 = 0.5;

        assert_eq!(12.0, m.determinant());
    }

    ///
    /// a singular matrix has two identical rows; determinant is zero
    ///
    #[test]
    fn test_singular() {
        let mut m = Matrix4x4::identity();
        m.r2c1 = m.r1c1;
        m.r2c2 = m.r1c2;
        m.r2c3 = m.r1c3;
        m.r2c4 = m.r1c4;

        assert_eq!(0.0, m.determinant());
    }

    #[test]
    fn test_scenario1() {
        let m = Matrix4x4 {
            r1c1: 1.0, r1c2: 0.0, r1c3: 2.0, r1c4: -1.0,
            r2c1: 3.0, r2c2: 0.0, r2c3: 0.5, r2c4: 2.0,
            r3c1: 2.0, r3c2: 1.0, r3c3: 4.0, r3c4: 0.0,
            r4c1: 0.0, r4c2: 0.0, r4c3: 3.0, r4c4: 1.0,
        };

        assert_eq!(20.5, m.determinant());
    }
}
