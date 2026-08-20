use crate::geometry::orient::matrix::m3x3::Matrix3x3;
use crate::geometry::orient::matrix::m4x4::Matrix4x4;

pub mod m3x3;
pub mod m4x4;

pub fn extract_rotation(world: &Matrix4x4) -> Matrix3x3 {
    Matrix3x3 {
        c1r1: world.c1r1, c2r1: world.c2r1, c3r1: world.c3r1,
        c1r2: world.c1r2, c2r2: world.c2r2, c3r2: world.c3r2,
        c1r3: world.c1r3, c2r3: world.c2r3, c3r3: world.c3r3,
    }
}

pub fn rotate(world: &Matrix4x4, rotation: &Matrix3x3) -> Matrix4x4 {
    let current = extract_rotation(world);
    let combined = rotation.multiply(&current);
    Matrix4x4 {
        c1r1: combined.c1r1, c2r1: combined.c2r1, c3r1: combined.c3r1, c4r1: world.c4r1,
        c1r2: combined.c1r2, c2r2: combined.c2r2, c3r2: combined.c3r2, c4r2: world.c4r2,
        c1r3: combined.c1r3, c2r3: combined.c2r3, c3r3: combined.c3r3, c4r3: world.c4r3,
        c1r4: world.c1r4,       c2r4: world.c2r4,       c3r4: world.c3r4,       c4r4: world.c4r4,
    }
}
