use crate::geometry::orient::matrix::m4x4::Matrix4x4;

impl Matrix4x4 {
    pub fn normalize(&mut self) {
        if let Some(x) = normalize(self) {
            *self = x;
        }
    }
}

pub fn normalize(matrix: &Matrix4x4) -> Option<Matrix4x4> {
    let det = matrix.determinant();
    if det == 0.0 || det.is_nan() {
        return None;
    }

    let scale = 1.0 / det.abs().powf(0.25) * det.signum();

    Some(Matrix4x4 {
        r1c1: matrix.r1c1 * scale,
        r1c2: matrix.r1c2 * scale,
        r1c3: matrix.r1c3 * scale,
        r1c4: matrix.r1c4 * scale,
        r2c1: matrix.r2c1 * scale,
        r2c2: matrix.r2c2 * scale,
        r2c3: matrix.r2c3 * scale,
        r2c4: matrix.r2c4 * scale,
        r3c1: matrix.r3c1 * scale,
        r3c2: matrix.r3c2 * scale,
        r3c3: matrix.r3c3 * scale,
        r3c4: matrix.r3c4 * scale,
        r4c1: matrix.r4c1 * scale,
        r4c2: matrix.r4c2 * scale,
        r4c3: matrix.r4c3 * scale,
        r4c4: matrix.r4c4 * scale,
    })
}

#[cfg(test)]
mod tests {
    use crate::geometry::orient::matrix::m4x4::Matrix4x4;
    use crate::geometry::orient::matrix::m4x4::norm::normalize;

    fn test_identity() {
        let m = Matrix4x4::identity();
        let n = normalize(&m).unwrap();

        assert_eq!(m, n);
        assert_eq!(1.0, n.determinant());
    }

    #[test]
    fn test_scaled() {
        let mut m = Matrix4x4::identity();
        m.r1c1 = 3.0;
        m.r2c2 = 3.0;
        m.r3c3 = 3.0;
        m.r4c4 = 3.0;

        assert_eq!(81.0, m.determinant());
    }

    ///
    /// a row of zeros results in determinant of 0.
    ///
    #[test]
    fn test_singular() {
        let mut m = Matrix4x4::identity();
        m.r1c1 = 0.0;
        m.r1c2 = 0.0;
        m.r1c3 = 0.0;
        m.r1c4 = 0.0;

        assert_eq!(0.0, m.determinant());
        assert!(normalize(&m).is_none());
    }
}
