pub(crate) mod multiply_scalar;
pub(crate) mod add;
pub(crate) mod subtract;
pub(crate) mod negate;
pub(crate) mod magnitude;
pub(crate) mod distance;
pub(crate) mod init;
pub(crate) mod divide_scalar;
pub(crate) mod cross_product;
pub(crate) mod dot_product;
pub(crate) mod normalize;
pub(crate) mod equal;

#[derive(Clone)]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
