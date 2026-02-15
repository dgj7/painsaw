use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;

///
/// multiply two matrices.
///
#[allow(unused)]// todo: remove this
pub fn multiply(left: &Matrix4x4, right: &Matrix4x4) -> Matrix4x4 {
    Matrix4x4 {
        c1r1: left.c1r1* right.c1r1 + left.c2r1* right.c1r2 + left.c3r1* right.c1r3 + left.c4r1* right.c1r4,
        c1r2: left.c1r2* right.c1r1 + left.c2r2* right.c1r2 + left.c3r2* right.c1r3 + left.c4r2* right.c1r4,
        c1r3: left.c1r3* right.c1r1 + left.c2r3* right.c1r2 + left.c3r3* right.c1r3 + left.c4r3* right.c1r4,
        c1r4: left.c1r4* right.c1r1 + left.c2r4* right.c1r2 + left.c3r4* right.c1r3 + left.c4r4* right.c1r4,

        c2r1: left.c1r1* right.c2r1 + left.c2r1* right.c2r2 + left.c3r1* right.c2r3 + left.c4r1* right.c2r4,
        c2r2: left.c1r2* right.c2r1 + left.c2r2* right.c2r2 + left.c3r2* right.c2r3 + left.c4r2* right.c2r4,
        c2r3: left.c1r3* right.c2r1 + left.c2r3* right.c2r2 + left.c3r3* right.c2r3 + left.c4r3* right.c2r4,
        c2r4: left.c1r4* right.c2r1 + left.c2r4* right.c2r2 + left.c3r4* right.c2r3 + left.c4r4* right.c2r4,

        c3r1: left.c1r1* right.c3r1 + left.c2r1* right.c3r2 + left.c3r1* right.c3r3 + left.c4r1* right.c3r4,
        c3r2: left.c1r2* right.c3r1 + left.c2r2* right.c3r2 + left.c3r2* right.c3r3 + left.c4r2* right.c3r4,
        c3r3: left.c1r3* right.c3r1 + left.c2r3* right.c3r2 + left.c3r3* right.c3r3 + left.c4r3* right.c3r4,
        c3r4: left.c1r4* right.c3r1 + left.c2r4* right.c3r2 + left.c3r4* right.c3r3 + left.c4r4* right.c3r4,

        c4r1: left.c1r1* right.c4r1 + left.c2r1* right.c4r2 + left.c3r1* right.c4r3 + left.c4r1* right.c4r4,
        c4r2: left.c1r2* right.c4r1 + left.c2r2* right.c4r2 + left.c3r2* right.c4r3 + left.c4r2* right.c4r4,
        c4r3: left.c1r3* right.c4r1 + left.c2r3* right.c4r2 + left.c3r3* right.c4r3 + left.c4r3* right.c4r4,
        c4r4: left.c1r4* right.c4r1 + left.c2r4* right.c4r2 + left.c3r4* right.c4r3 + left.c4r4* right.c4r4,
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::geometry::orient::matrix::m4x4::Matrix4x4;
    use crate::graphics::geometry::orient::matrix::m4x4::mult::multiply;

    #[test]
    fn test_mult_by_identity() {
        let identity = Matrix4x4::identity();
        let input = Matrix4x4 {
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

        let result = multiply(&input, &identity);

        /* expected that A*identity=A */
        assert_eq!(input, result);
    }
}
