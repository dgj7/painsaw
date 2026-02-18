use crate::geometry::orient::matrix::m4x4::Matrix4x4;

///
/// invert the given matrix, as long as it's invertible (having determinant > 0).
///
#[allow(dead_code)]// todo: remove this
fn invert(m: &Matrix4x4) -> Option<Matrix4x4> {
    /* calculate cofactors */
    let c11 = m.c2r2 * (m.c3r3 * m.c4r4 - m.c4r3 * m.c3r4) - m.c3r2 * (m.c2r3 * m.c4r4 - m.c4r3 * m.c2r4) + m.c4r2 * (m.c2r3 * m.c3r4 - m.c3r3 * m.c2r4);
    let c12 = -(m.c1r2 * (m.c3r3 * m.c4r4 - m.c4r3 * m.c3r4) - m.c3r2 * (m.c1r3 * m.c4r4 - m.c4r3 * m.c1r4) + m.c4r2 * (m.c1r3 * m.c3r4 - m.c3r3 * m.c1r4));
    let c13 = m.c1r2 * (m.c2r3 * m.c4r4 - m.c4r3 * m.c2r4) - m.c2r2 * (m.c1r3 * m.c4r4 - m.c4r3 * m.c1r4) + m.c4r2 * (m.c1r3 * m.c2r4 - m.c2r3 * m.c1r4);
    let c14 = -(m.c1r2 * (m.c2r3 * m.c3r4 - m.c3r3 * m.c2r4) - m.c2r2 * (m.c1r3 * m.c3r4 - m.c3r3 * m.c1r4) + m.c3r2 * (m.c1r3 * m.c2r4 - m.c2r3 * m.c1r4));
    let c21 = -(m.c2r1 * (m.c3r3 * m.c4r4 - m.c4r3 * m.c3r4) - m.c3r1 * (m.c2r3 * m.c4r4 - m.c4r3 * m.c2r4) + m.c4r1 * (m.c2r3 * m.c3r4 - m.c3r3 * m.c2r4));
    let c22 = m.c1r1 * (m.c3r3 * m.c4r4 - m.c4r3 * m.c3r4) - m.c3r1 * (m.c1r3 * m.c4r4 - m.c4r3 * m.c1r4) + m.c4r1 * (m.c1r3 * m.c3r4 - m.c3r3 * m.c1r4);
    let c23 = -(m.c1r1 * (m.c2r3 * m.c4r4 - m.c4r3 * m.c2r4) - m.c2r1 * (m.c1r3 * m.c4r4 - m.c4r3 * m.c1r4) + m.c4r1 * (m.c1r3 * m.c2r4 - m.c2r3 * m.c1r4));
    let c24 = m.c1r1 * (m.c2r3 * m.c3r4 - m.c3r3 * m.c2r4) - m.c2r1 * (m.c1r3 * m.c3r4 - m.c3r3 * m.c1r4) + m.c3r1 * (m.c1r3 * m.c2r4 - m.c2r3 * m.c1r4);
    let c31 = m.c2r1 * (m.c3r2 * m.c4r4 - m.c4r2 * m.c3r4) - m.c3r1 * (m.c2r2 * m.c4r4 - m.c4r2 * m.c2r4) + m.c4r1 * (m.c2r2 * m.c3r4 - m.c3r2 * m.c2r4);
    let c32 = -(m.c1r1 * (m.c3r2 * m.c4r4 - m.c4r2 * m.c3r4) - m.c3r1 * (m.c1r2 * m.c4r4 - m.c4r2 * m.c1r4) + m.c4r1 * (m.c1r2 * m.c3r4 - m.c3r2 * m.c1r4));
    let c33 = m.c1r1 * (m.c2r2 * m.c4r4 - m.c4r2 * m.c2r4) - m.c2r1 * (m.c1r2 * m.c4r4 - m.c4r2 * m.c1r4) + m.c4r1 * (m.c1r2 * m.c2r4 - m.c2r2 * m.c1r4);
    let c34 = -(m.c1r1 * (m.c2r2 * m.c3r4 - m.c3r2 * m.c2r4) - m.c2r1 * (m.c1r2 * m.c3r4 - m.c3r2 * m.c1r4) + m.c3r1 * (m.c1r2 * m.c2r4 - m.c2r2 * m.c1r4));
    let c41 = -(m.c2r1 * (m.c3r2 * m.c4r3 - m.c4r2 * m.c3r3) - m.c3r1 * (m.c2r2 * m.c4r3 - m.c4r2 * m.c2r3) + m.c4r1 * (m.c2r2 * m.c3r3 - m.c3r2 * m.c2r3));
    let c42 = m.c1r1 * (m.c3r2 * m.c4r3 - m.c4r2 * m.c3r3) - m.c3r1 * (m.c1r2 * m.c4r3 - m.c4r2 * m.c1r3) + m.c4r1 * (m.c1r2 * m.c3r3 - m.c3r2 * m.c1r3);
    let c43 = -(m.c1r1 * (m.c2r2 * m.c4r3 - m.c4r2 * m.c2r3) - m.c2r1 * (m.c1r2 * m.c4r3 - m.c4r2 * m.c1r3) + m.c4r1 * (m.c1r2 * m.c2r3 - m.c2r2 * m.c1r3));
    let c44 = m.c1r1 * (m.c2r2 * m.c3r3 - m.c3r2 * m.c2r3) - m.c2r1 * (m.c1r2 * m.c3r3 - m.c3r2 * m.c1r3) + m.c3r1 * (m.c1r2 * m.c2r3 - m.c2r2 * m.c1r3);

    /* calculate determinant; consists of the first column and its cofactors */
    let determinant = m.c1r1 * c11 + m.c2r1 * c12 + m.c3r1 * c13 + m.c4r1 * c14;

    /* if the deterrent is practically zero, return none */
    if determinant.abs() < f32::EPSILON {
        return None;
    }

    /* otherwise, get the inverse determinant */
    let inv_det = 1.0 / determinant;

    /* return a matrix, comprised of each cofactor multiplied by the inverse determinant */
    Some(Matrix4x4 {
        c1r1: c11 * inv_det,
        c1r2: c12 * inv_det,
        c1r3: c13 * inv_det,
        c1r4: c14 * inv_det,

        c2r1: c21 * inv_det,
        c2r2: c22 * inv_det,
        c2r3: c23 * inv_det,
        c2r4: c24 * inv_det,

        c3r1: c31 * inv_det,
        c3r2: c32 * inv_det,
        c3r3: c33 * inv_det,
        c3r4: c34 * inv_det,

        c4r1: c41 * inv_det,
        c4r2: c42 * inv_det,
        c4r3: c43 * inv_det,
        c4r4: c44 * inv_det,
    })
}

#[cfg(test)]
mod tests {
    use crate::geometry::orient::matrix::m4x4::invert::invert;
    use crate::geometry::orient::matrix::m4x4::Matrix4x4;
    use crate::geometry::orient::matrix::m4x4::mult::multiply;

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

        assert!(is_near(matrix.c1r1, inverse.c1r1));
        assert!(is_near(matrix.c1r2, inverse.c1r2));
        assert!(is_near(matrix.c1r3, inverse.c1r3));
        assert!(is_near(matrix.c1r4, inverse.c1r4));

        assert!(is_near(matrix.c2r1, inverse.c2r1));
        assert!(is_near(matrix.c2r2, inverse.c2r2));
        assert!(is_near(matrix.c2r1, inverse.c2r3));
        assert!(is_near(matrix.c2r1, inverse.c2r4));

        assert!(is_near(matrix.c3r1, inverse.c3r1));
        assert!(is_near(matrix.c3r2, inverse.c3r2));
        assert!(is_near(matrix.c3r3, inverse.c3r3));
        assert!(is_near(matrix.c3r4, inverse.c3r4));

        assert!(is_near(matrix.c4r1, inverse.c4r1));
        assert!(is_near(matrix.c4r2, inverse.c4r2));
        assert!(is_near(matrix.c4r3, inverse.c4r3));
        assert!(is_near(matrix.c4r4, inverse.c4r4));
    }

    ///
    /// a matrix times its own inverse equals identity matrix
    ///
    #[test]
    fn test_inverse_1() {
        let matrix = Matrix4x4 {
            c1r1: 1.0,
            c1r2: 0.0,
            c1r3: 1.0,
            c1r4: 0.0,
            c2r1: 0.0,
            c2r2: 3.0,
            c2r3: 0.0,
            c2r4: 2.0,
            c3r1: 2.0,
            c3r2: 0.0,
            c3r3: 1.0,
            c3r4: 0.0,
            c4r1: 0.0,
            c4r2: 4.0,
            c4r3: 0.0,
            c4r4: 3.0,
        };
        let inverse = invert(&matrix).expect("couldn't invert matrix");

        let result = multiply(&matrix, &inverse);
        let expected = Matrix4x4::identity();

        assert!(is_near(expected.c1r1, result.c1r1));
        assert!(is_near(expected.c1r2, result.c1r2));
        assert!(is_near(expected.c1r3, result.c1r3));
        assert!(is_near(expected.c1r4, result.c1r4));

        assert!(is_near(expected.c2r1, result.c2r1));
        assert!(is_near(expected.c2r2, result.c2r2));
        assert!(is_near(expected.c2r1, result.c2r3));
        assert!(is_near(expected.c2r1, result.c2r4));

        assert!(is_near(expected.c3r1, result.c3r1));
        assert!(is_near(expected.c3r2, result.c3r2));
        assert!(is_near(expected.c3r3, result.c3r3));
        assert!(is_near(expected.c3r4, result.c3r4));

        assert!(is_near(expected.c4r1, result.c4r1));
        assert!(is_near(expected.c4r2, result.c4r2));
        assert!(is_near(expected.c4r3, result.c4r3));
        assert!(is_near(expected.c4r4, result.c4r4));
    }

    ///
    /// test that a singular matrix won't be inverted
    ///
    #[test]
    fn test_inverse_singular_1() {
        let matrix = Matrix4x4 {
            c1r1: 1.0,
            c1r2: 5.0,
            c1r3: 9.0,
            c1r4: 13.0,
            c2r1: 2.0,
            c2r2: 6.0,
            c2r3: 10.0,
            c2r4: 14.0,
            c3r1: 3.0,
            c3r2: 7.0,
            c3r3: 11.0,
            c3r4: 15.0,
            c4r1: 4.0,
            c4r2: 8.0,
            c4r3: 12.0,
            c4r4: 16.0,
        };
        assert!(invert(&matrix).is_none());
    }
}
