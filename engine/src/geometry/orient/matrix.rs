use crate::geometry::orient::matrix::m3x3::Matrix3x3;
use crate::geometry::orient::matrix::m4x4::Matrix4x4;

pub mod m3x3;
pub mod m4x4;

pub fn extract_rotation(world: &Matrix4x4) -> Matrix3x3 {
    Matrix3x3 {
        r1c1: world.r1c1, r1c2: world.r1c2, r1c3: world.r1c3,
        r2c1: world.r2c1, r2c2: world.r2c2, r2c3: world.r2c3,
        r3c1: world.r3c1, r3c2: world.r3c2, r3c3: world.r3c3,
    }
}

pub fn rotate(world: &Matrix4x4, rotation: &Matrix3x3) -> Matrix4x4 {
    let current = extract_rotation(world);
    let combined = rotation.multiply(&current);
    Matrix4x4 {
        r1c1: combined.r1c1, r1c2: combined.r1c2, r1c3: combined.r1c3, r1c4: world.r1c4,
        r2c1: combined.r2c1, r2c2: combined.r2c2, r2c3: combined.r2c3, r2c4: world.r2c4,
        r3c1: combined.r3c1, r3c2: combined.r3c2, r3c3: combined.r3c3, r3c4: world.r3c4,
        r4c1: world.r4c1, r4c2: world.r4c2, r4c3: world.r4c3, r4c4: world.r4c4,
    }
}
