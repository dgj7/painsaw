use crate::geometry::orient::matrix::m4x4::Matrix4x4;

///
/// invert the given matrix, as long as it's invertible (having determinant > 0).
///
fn invert(m: &Matrix4x4) -> Option<Matrix4x4> {
    /* calculate cofactors */
    let c11 = m.r2c2 * (m.r3c3 * m.r4c4 - m.r3c4 * m.r4c3)
        - m.r2c3 * (m.r3c2 * m.r4c4 - m.r3c4 * m.r4c2)
        + m.r2c4 * (m.r3c2 * m.r4c3 - m.r3c3 * m.r4c2);
    let c12 = -(m.r2c1 * (m.r3c3 * m.r4c4 - m.r3c4 * m.r4c3)
        - m.r2c3 * (m.r3c1 * m.r4c4 - m.r3c4 * m.r4c1)
        + m.r2c4 * (m.r3c1 * m.r4c3 - m.r3c3 * m.r4c1));
    let c13 = m.r2c1 * (m.r3c2 * m.r4c4 - m.r3c4 * m.r4c2)
        - m.r2c2 * (m.r3c1 * m.r4c4 - m.r3c4 * m.r4c1)
        + m.r2c4 * (m.r3c1 * m.r4c2 - m.r3c2 * m.r4c1);
    let c14 = -(m.r2c1 * (m.r3c2 * m.r4c3 - m.r3c3 * m.r4c2)
        - m.r2c2 * (m.r3c1 * m.r4c3 - m.r3c3 * m.r4c1)
        + m.r2c3 * (m.r3c1 * m.r4c2 - m.r3c2 * m.r4c1));
    let c21 = -(m.r1c2 * (m.r3c3 * m.r4c4 - m.r3c4 * m.r4c3)
        - m.r1c3 * (m.r3c2 * m.r4c4 - m.r3c4 * m.r4c2)
        + m.r1c4 * (m.r3c2 * m.r4c3 - m.r3c3 * m.r4c2));
    let c22 = m.r1c1 * (m.r3c3 * m.r4c4 - m.r3c4 * m.r4c3)
        - m.r1c3 * (m.r3c1 * m.r4c4 - m.r3c4 * m.r4c1)
        + m.r1c4 * (m.r3c1 * m.r4c3 - m.r3c3 * m.r4c1);
    let c23 = -(m.r1c1 * (m.r3c2 * m.r4c4 - m.r3c4 * m.r4c2)
        - m.r1c2 * (m.r3c1 * m.r4c4 - m.r3c4 * m.r4c1)
        + m.r1c4 * (m.r3c1 * m.r4c2 - m.r3c2 * m.r4c1));
    let c24 = m.r1c1 * (m.r3c2 * m.r4c3 - m.r3c3 * m.r4c2)
        - m.r1c2 * (m.r3c1 * m.r4c3 - m.r3c3 * m.r4c1)
        + m.r1c3 * (m.r3c1 * m.r4c2 - m.r3c2 * m.r4c1);
    let c31 = m.r1c2 * (m.r2c3 * m.r4c4 - m.r2c4 * m.r4c3)
        - m.r1c3 * (m.r2c2 * m.r4c4 - m.r2c4 * m.r4c2)
        + m.r1c4 * (m.r2c2 * m.r4c3 - m.r2c3 * m.r4c2);
    let c32 = -(m.r1c1 * (m.r2c3 * m.r4c4 - m.r2c4 * m.r4c3)
        - m.r1c3 * (m.r2c1 * m.r4c4 - m.r2c4 * m.r4c1)
        + m.r1c4 * (m.r2c1 * m.r4c3 - m.r2c3 * m.r4c1));
    let c33 = m.r1c1 * (m.r2c2 * m.r4c4 - m.r2c4 * m.r4c2)
        - m.r1c2 * (m.r2c1 * m.r4c4 - m.r2c4 * m.r4c1)
        + m.r1c4 * (m.r2c1 * m.r4c2 - m.r2c2 * m.r4c1);
    let c34 = -(m.r1c1 * (m.r2c2 * m.r4c3 - m.r2c3 * m.r4c2)
        - m.r1c2 * (m.r2c1 * m.r4c3 - m.r2c3 * m.r4c1)
        + m.r1c3 * (m.r2c1 * m.r4c2 - m.r2c2 * m.r4c1));
    let c41 = -(m.r1c2 * (m.r2c3 * m.r3c4 - m.r2c4 * m.r3c3)
        - m.r1c3 * (m.r2c2 * m.r3c4 - m.r2c4 * m.r3c2)
        + m.r1c4 * (m.r2c2 * m.r3c3 - m.r2c3 * m.r3c2));
    let c42 = m.r1c1 * (m.r2c3 * m.r3c4 - m.r2c4 * m.r3c3)
        - m.r1c3 * (m.r2c1 * m.r3c4 - m.r2c4 * m.r3c1)
        + m.r1c4 * (m.r2c1 * m.r3c3 - m.r2c3 * m.r3c1);
    let c43 = -(m.r1c1 * (m.r2c2 * m.r3c4 - m.r2c4 * m.r3c2)
        - m.r1c2 * (m.r2c1 * m.r3c4 - m.r2c4 * m.r3c1)
        + m.r1c4 * (m.r2c1 * m.r3c2 - m.r2c2 * m.r3c1));
    let c44 = m.r1c1 * (m.r2c2 * m.r3c3 - m.r2c3 * m.r3c2)
        - m.r1c2 * (m.r2c1 * m.r3c3 - m.r2c3 * m.r3c1)
        + m.r1c3 * (m.r2c1 * m.r3c2 - m.r2c2 * m.r3c1);

    /* calculate determinant; consists of the first column and its cofactors */
    let determinant = m.r1c1 * c11 + m.r1c2 * c12 + m.r1c3 * c13 + m.r1c4 * c14;

    /* if the deterrent is practically zero, return none */
    if determinant.abs() < f32::EPSILON {
        return None;
    }

    /* otherwise, get the inverse determinant */
    let inv_det = 1.0 / determinant;

    /* return a matrix, comprised of each cofactor multiplied by the inverse determinant */
    Some(Matrix4x4 {
        r1c1: c11 * inv_det,
        r2c1: c12 * inv_det,
        r3c1: c13 * inv_det,
        r4c1: c14 * inv_det,

        r1c2: c21 * inv_det,
        r2c2: c22 * inv_det,
        r3c2: c23 * inv_det,
        r4c2: c24 * inv_det,

        r1c3: c31 * inv_det,
        r2c3: c32 * inv_det,
        r3c3: c33 * inv_det,
        r4c3: c34 * inv_det,

        r1c4: c41 * inv_det,
        r2c4: c42 * inv_det,
        r3c4: c43 * inv_det,
        r4c4: c44 * inv_det,
    })
}

#[cfg(test)]
mod tests {
    use crate::geometry::orient::matrix::m4x4::invert::invert;
    use crate::geometry::orient::matrix::m4x4::mult::multiply;
    use crate::geometry::orient::matrix::m4x4::Matrix4x4;

    fn is_near(left: f32, right: f32) -> bool {
        (left - right).abs() <= f32::EPSILON
    }

    ///
    /// inverse of identity should be _very close_ to the identity.
    ///
    #[test]
    fn test_inverse_identity() {
        let matrix = Matrix4x4::identity();
        let inverse = invert(&matrix).unwrap();

        assert!(is_near(matrix.r1c1, inverse.r1c1));
        assert!(is_near(matrix.r2c1, inverse.r2c1));
        assert!(is_near(matrix.r3c1, inverse.r3c1));
        assert!(is_near(matrix.r4c1, inverse.r4c1));

        assert!(is_near(matrix.r1c2, inverse.r1c2));
        assert!(is_near(matrix.r2c2, inverse.r2c2));
        assert!(is_near(matrix.r1c2, inverse.r3c2));
        assert!(is_near(matrix.r1c2, inverse.r4c2));

        assert!(is_near(matrix.r1c3, inverse.r1c3));
        assert!(is_near(matrix.r2c3, inverse.r2c3));
        assert!(is_near(matrix.r3c3, inverse.r3c3));
        assert!(is_near(matrix.r4c3, inverse.r4c3));

        assert!(is_near(matrix.r1c4, inverse.r1c4));
        assert!(is_near(matrix.r2c4, inverse.r2c4));
        assert!(is_near(matrix.r3c4, inverse.r3c4));
        assert!(is_near(matrix.r4c4, inverse.r4c4));
    }

    ///
    /// a matrix times its own inverse equals identity matrix
    ///
    #[test]
    fn test_inverse_1() {
        let matrix = Matrix4x4 {
            r1c1: 1.0,
            r2c1: 0.0,
            r3c1: 1.0,
            r4c1: 0.0,
            r1c2: 0.0,
            r2c2: 3.0,
            r3c2: 0.0,
            r4c2: 2.0,
            r1c3: 2.0,
            r2c3: 0.0,
            r3c3: 1.0,
            r4c3: 0.0,
            r1c4: 0.0,
            r2c4: 4.0,
            r3c4: 0.0,
            r4c4: 3.0,
        };
        let inverse = invert(&matrix).expect("couldn't invert matrix");

        let result = multiply(&matrix, &inverse);
        let expected = Matrix4x4::identity();

        assert!(is_near(expected.r1c1, result.r1c1));
        assert!(is_near(expected.r2c1, result.r2c1));
        assert!(is_near(expected.r3c1, result.r3c1));
        assert!(is_near(expected.r4c1, result.r4c1));

        assert!(is_near(expected.r1c2, result.r1c2));
        assert!(is_near(expected.r2c2, result.r2c2));
        assert!(is_near(expected.r1c2, result.r3c2));
        assert!(is_near(expected.r1c2, result.r4c2));

        assert!(is_near(expected.r1c3, result.r1c3));
        assert!(is_near(expected.r2c3, result.r2c3));
        assert!(is_near(expected.r3c3, result.r3c3));
        assert!(is_near(expected.r4c3, result.r4c3));

        assert!(is_near(expected.r1c4, result.r1c4));
        assert!(is_near(expected.r2c4, result.r2c4));
        assert!(is_near(expected.r3c4, result.r3c4));
        assert!(is_near(expected.r4c4, result.r4c4));
    }

    ///
    /// test that a singular matrix won't be inverted
    ///
    #[test]
    fn test_inverse_singular_1() {
        let matrix = Matrix4x4 {
            r1c1: 1.0,
            r2c1: 5.0,
            r3c1: 9.0,
            r4c1: 13.0,
            r1c2: 2.0,
            r2c2: 6.0,
            r3c2: 10.0,
            r4c2: 14.0,
            r1c3: 3.0,
            r2c3: 7.0,
            r3c3: 11.0,
            r4c3: 15.0,
            r1c4: 4.0,
            r2c4: 8.0,
            r3c4: 12.0,
            r4c4: 16.0,
        };
        assert!(invert(&matrix).is_none());
    }
}
