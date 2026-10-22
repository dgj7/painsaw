use crate::geometry::orient::matrix::m4x4::Matrix4x4;

///
/// multiply two matrices.
///
pub fn multiply(left: &Matrix4x4, right: &Matrix4x4) -> Matrix4x4 {
    Matrix4x4 {
        r1c1: left.r1c1 * right.r1c1 + left.r1c2 * right.r2c1 + left.r1c3 * right.r3c1 + left.r1c4 * right.r4c1,
        r2c1: left.r2c1 * right.r1c1 + left.r2c2 * right.r2c1 + left.r2c3 * right.r3c1 + left.r2c4 * right.r4c1,
        r3c1: left.r3c1 * right.r1c1 + left.r3c2 * right.r2c1 + left.r3c3 * right.r3c1 + left.r3c4 * right.r4c1,
        r4c1: left.r4c1 * right.r1c1 + left.r4c2 * right.r2c1 + left.r4c3 * right.r3c1 + left.r4c4 * right.r4c1,

        r1c2: left.r1c1 * right.r1c2 + left.r1c2 * right.r2c2 + left.r1c3 * right.r3c2 + left.r1c4 * right.r4c2,
        r2c2: left.r2c1 * right.r1c2 + left.r2c2 * right.r2c2 + left.r2c3 * right.r3c2 + left.r2c4 * right.r4c2,
        r3c2: left.r3c1 * right.r1c2 + left.r3c2 * right.r2c2 + left.r3c3 * right.r3c2 + left.r3c4 * right.r4c2,
        r4c2: left.r4c1 * right.r1c2 + left.r4c2 * right.r2c2 + left.r4c3 * right.r3c2 + left.r4c4 * right.r4c2,

        r1c3: left.r1c1 * right.r1c3 + left.r1c2 * right.r2c3 + left.r1c3 * right.r3c3 + left.r1c4 * right.r4c3,
        r2c3: left.r2c1 * right.r1c3 + left.r2c2 * right.r2c3 + left.r2c3 * right.r3c3 + left.r2c4 * right.r4c3,
        r3c3: left.r3c1 * right.r1c3 + left.r3c2 * right.r2c3 + left.r3c3 * right.r3c3 + left.r3c4 * right.r4c3,
        r4c3: left.r4c1 * right.r1c3 + left.r4c2 * right.r2c3 + left.r4c3 * right.r3c3 + left.r4c4 * right.r4c3,

        r1c4: left.r1c1 * right.r1c4 + left.r1c2 * right.r2c4 + left.r1c3 * right.r3c4 + left.r1c4 * right.r4c4,
        r2c4: left.r2c1 * right.r1c4 + left.r2c2 * right.r2c4 + left.r2c3 * right.r3c4 + left.r2c4 * right.r4c4,
        r3c4: left.r3c1 * right.r1c4 + left.r3c2 * right.r2c4 + left.r3c3 * right.r3c4 + left.r3c4 * right.r4c4,
        r4c4: left.r4c1 * right.r1c4 + left.r4c2 * right.r2c4 + left.r4c3 * right.r3c4 + left.r4c4 * right.r4c4,
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::orient::matrix::m4x4::mult::multiply;
    use crate::geometry::orient::matrix::m4x4::Matrix4x4;

    #[test]
    fn test_mult_by_identity() {
        let identity = Matrix4x4::identity();
        let input = Matrix4x4 {
            r1c1: 1.0, r1c2: 2.0, r1c3: 3.0, r1c4: 4.0,
            r2c1: 5.0, r2c2: 6.0, r2c3: 7.0, r2c4: 8.0,
            r3c1: 9.0, r3c2: 10.0, r3c3: 11.0, r3c4: 12.0,
            r4c1: 13.0, r4c2: 14.0, r4c3: 15.0, r4c4: 16.0,
        };

        let result = multiply(&input, &identity);

        /* expected that A*identity=A */
        assert_eq!(input, result);
    }

    #[test]
    fn test_scenario1() {
        let m1 = Matrix4x4 {
            r1c1: 1.0,  r1c2: 2.0,  r1c3: 3.0,  r1c4: 4.0,
            r2c1: 5.0,  r2c2: 6.0,  r2c3: 7.0,  r2c4: 8.0,
            r3c1: 9.0,  r3c2: 10.0, r3c3: 11.0, r3c4: 12.0,
            r4c1: 13.0, r4c2: 14.0, r4c3: 15.0, r4c4: 16.0,
        };
        let m2 = Matrix4x4 {
            r1c1: 2.0, r1c2: 0.0, r1c3: 1.0, r1c4: 3.0,
            r2c1: 1.0, r2c2: 4.0, r2c3: 0.0, r2c4: 1.0,
            r3c1: 0.0, r3c2: 1.0, r3c3: 2.0, r3c4: 0.0,
            r4c1: 3.0, r4c2: 0.0, r4c3: 1.0, r4c4: 2.0,
        };
        let expected = Matrix4x4 {
            r1c1: 16.0,  r1c2: 11.0, r1c3: 11.0, r1c4: 13.0,
            r2c1: 40.0,  r2c2: 31.0, r2c3: 27.0, r2c4: 37.0,
            r3c1: 64.0,  r3c2: 51.0, r3c3: 43.0, r3c4: 61.0,
            r4c1: 88.0,  r4c2: 71.0, r4c3: 59.0, r4c4: 85.0,
        };

        let result = multiply(&m1, &m2);

        assert_eq!(expected, result);

    }
}
