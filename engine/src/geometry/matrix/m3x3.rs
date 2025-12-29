use num_traits::Float;

pub struct Matrix3x3<F: Float> {
    pub m11: F,
    pub m12: F,
    pub m13: F,

    pub m21: F,
    pub m22: F,
    pub m23: F,

    pub m31: F,
    pub m32: F,
    pub m33: F,
}

#[allow(dead_code)]
type RotationMatrix<F> = Matrix3x3<F>;
