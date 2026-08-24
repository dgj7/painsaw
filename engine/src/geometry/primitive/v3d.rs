pub(crate) mod mult_scalar;
pub(crate) mod add;
pub(crate) mod subtract;
pub(crate) mod negate;
pub(crate) mod magnitude;
mod distance;

#[derive(Clone)]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex3D {
    pub fn is_equal(&self, other: &Vertex3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    pub fn normalize(&mut self) {
        let magnitude_squared = self.x * self.x + self.y * self.y + self.z * self.z;
        if magnitude_squared > 0.0 {
            let one_div_mag = 1.0 / magnitude_squared.sqrt();
            self.x = self.x * one_div_mag;
            self.y = self.y * one_div_mag;
            self.z = self.z * one_div_mag;
        }
    }

    ///
    /// compute the dot product
    ///
    /// dot product: "how much" two vectors point in the same direction.
    /// positive: same direction
    /// zero: perpendicular
    /// negative: opposite directions
    ///
    /// computed as:
    /// absolute value of left
    /// x absolute value of right
    /// x cosine of the angle betwixt them
    ///
    pub fn dot_product(&mut self, multiplier: &Vertex3D) -> f32 {
        self.x * multiplier.x + self.y * multiplier.y + self.z * multiplier.z
    }

    pub fn cross_product(&mut self, multiplier: &Vertex3D) {
        self.x = self.y * multiplier.z - self.z * multiplier.y;
        self.y = self.z * multiplier.x - self.x * multiplier.z;
        self.z = self.x * multiplier.y - self.y * multiplier.x;
    }


}

///
/// functions that result in a new instance.
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

    pub fn new_div_scalar(dividend: &Vertex3D, divisor: f32) -> Vertex3D {
        Vertex3D {
            x: dividend.x / divisor,
            y: dividend.y / divisor,
            z: dividend.z / divisor,
        }
    }

    pub fn new_cross_product(left: &Vertex3D, right: &Vertex3D) -> Vertex3D {
        Vertex3D {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x,
        }
    }
}

///
/// test [v3d::distance()].
///
#[cfg(test)]
mod point3d_distance_tests {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test_positive1() {
        let left = Vertex3D::new(2.0, 3.0, 4.0);
        let right = Vertex3D::new(5.0, 7.0, 9.0);

        let dist = left.distance_to(&right);

        assert_eq!(7.0710678118654755, dist);
    }

    #[test]
    fn test_mixed1() {
        let left = Vertex3D::new(-3.0, 4.0, -2.0);
        let right = Vertex3D::new(1.0, -1.0, 3.0);

        let dist = left.distance_to(&right);

        assert_eq!(8.12403840463596, dist);
    }
}
