
#[derive(Clone)]
pub struct Vertex2D {
    pub x: f32,
    pub y: f32,
}

impl Vertex2D {
    pub fn new(x: f32, y: f32) -> Vertex2D {
        Vertex2D { x, y }
    }

    pub fn origin() -> Vertex2D {
        Vertex2D { x: 0.0, y: 0.0 }
    }

    pub fn distance(&self, other: &Vertex2D) -> f32 {
        ((self.x-other.x).powi(2) + (self.y-other.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod point2d_distance_tests {
    use crate::graphics::geometry::primitive::v2d::Vertex2D;

    #[test]
    fn test_positive1() {
        let left = Vertex2D::new(3.0, 1.0);
        let right = Vertex2D::new(6.0, 5.0);

        let dist = left.distance(&right);

        assert_eq!(5.0, dist);
    }

    #[test]
    fn test_mixed1() {
        let left = Vertex2D::new(-1.0, 3.0);
        let right = Vertex2D::new(2.0, -2.0);

        let dist = left.distance(&right);

        assert_eq!(5.830951894845301, dist);
    }

    #[test]
    fn test_mixed2() {
        let left = Vertex2D::new(2.0, -6.0);
        let right = Vertex2D::new(7.0, 3.0);

        let dist = left.distance(&right);

        assert_eq!(10.295630140987, dist);
    }
}
