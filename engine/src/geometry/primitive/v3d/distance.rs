use crate::geometry::primitive::v3d::Vertex3D;

///
/// methods related to calculating distances between vertices.
///
impl Vertex3D {
    pub fn distance_to(&self, other: &Vertex3D) -> f32 {
        distance(self, other)
    }
}

///
/// compute the distance between two 3d points.
///
pub fn distance(left: &Vertex3D, right: &Vertex3D) -> f32 {
    quadrance(left, right).sqrt()
}

///
/// compute the quadrance.  aka Euclidean distance or squared distance.
///
/// this is value before taking the square root and finding the distance.
///
pub fn quadrance(left: &Vertex3D, right: &Vertex3D) -> f32 {
    let dx = left.x - right.x;
    let dy = left.y - right.y;
    let dz = left.z - right.z;
    dx * dx + dy * dy + dz * dz
}

///
/// test [Vertex3D::distance_to()].
/// automatically tests [distance()] and [quadrance()].
///
#[cfg(test)]
mod test_vtx3d_distance {
    use crate::geometry::primitive::v3d::Vertex3D;

    #[test]
    fn test_both_positive() {
        let left = Vertex3D {
            x: 7.0,
            y: 4.0,
            z: 3.0,
        };
        let right = Vertex3D {
            x: 17.0,
            y: 6.0,
            z: 2.0,
        };

        assert_eq!(10.246951, left.distance_to(&right));
    }
}
