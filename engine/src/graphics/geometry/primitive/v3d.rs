
#[derive(Clone)]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex3D {

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

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

    pub fn distance_to(&self, other: &Vertex3D) -> f32 {
        distance(self, other)
    }

    pub fn add(&mut self, addend: &Vertex3D) {
        self.x = self.x + addend.x;
        self.y = self.y + addend.y;
        self.z = self.z + addend.z;

    }

    pub fn subtract(&mut self, subtrahend: &Vertex3D) {
        self.x = self.x - subtrahend.x;
        self.y = self.y - subtrahend.y;
        self.z = self.z - subtrahend.z;
    }

    pub fn dot_product(&mut self, multiplier: &Vertex3D) -> f32 {
        self.x * multiplier.x + self.y * multiplier.y + self.z * multiplier.z
    }
}

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

    pub fn new_add(left_addend: &Vertex3D, right_addend: &Vertex3D) -> Vertex3D {
        Vertex3D {
            x: left_addend.x + right_addend.x,
            y: left_addend.y + right_addend.y,
            z: left_addend.z + right_addend.z,
        }
    }

    pub fn new_subtract(minuend: &Vertex3D, subtrahend: &Vertex3D) -> Vertex3D {
        Vertex3D {
            x: minuend.x - subtrahend.x,
            y: minuend.y - subtrahend.y,
            z: minuend.z - subtrahend.z,
        }
    }

    pub fn new_mult_scalar(multiplicand: &Vertex3D, multiplier: f32) -> Vertex3D {
        Vertex3D {
            x: multiplicand.x * multiplier,
            y: multiplicand.y * multiplier,
            z: multiplicand.z * multiplier,
        }
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

pub fn magnitude(p: &Vertex3D) -> f32 {
    ((p.x*p.x) + (p.y*p.y) + (p.z*p.z)).sqrt()
}

pub fn distance_squared(left: &Vertex3D, right: &Vertex3D) -> f32 {
    let dx = left.x - right.x;
    let dy = left.y - right.y;
    let dz = left.z - right.z;
    dx * dx + dy * dy + dz * dz
}

pub fn distance(left: &Vertex3D, right: &Vertex3D) -> f32 {
    distance_squared(left, right).sqrt()
}

#[cfg(test)]
mod point3d_distance_tests {
    use crate::graphics::geometry::primitive::v3d::Vertex3D;

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
