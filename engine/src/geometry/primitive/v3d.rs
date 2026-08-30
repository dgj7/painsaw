pub(crate) mod add;
pub(crate) mod cross_product;
pub(crate) mod distance;
pub(crate) mod divide_scalar;
pub(crate) mod dot_product;
pub(crate) mod equal;
pub(crate) mod magnitude;
pub(crate) mod multiply_scalar;
pub(crate) mod negate;
pub(crate) mod normalize;
pub(crate) mod subtract;

///
/// Representation of a 3d point, vertex, or vector.
///
/// includes supporting methods for all 3 types.
///
#[derive(Clone)]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

///
/// methods related to initialization.
///
impl Vertex3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex3D {
        Vertex3D { x, y, z }
    }

    pub fn origin() -> Vertex3D {
        Vertex3D::new(0.0, 0.0, 0.0)
    }

    pub fn create_x_unit() -> Vertex3D {
        Vertex3D::new(1.0, 0.0, 0.0)
    }

    pub fn create_y_unit() -> Vertex3D {
        Vertex3D::new(0.0, 1.0, 0.0)
    }

    pub fn create_z_unit() -> Vertex3D {
        Vertex3D::new(0.0, 0.0, 1.0)
    }
}
