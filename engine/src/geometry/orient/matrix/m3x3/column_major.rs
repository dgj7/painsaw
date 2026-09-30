use crate::geometry::angle::radians::Radians;
use crate::geometry::orient::matrix::m3x3::Matrix3x3;
use crate::geometry::orient::matrix::m3x3::mult::multiply;

// todo: rename this to specify column_major
pub fn from_pitch_yaw_roll(pitch: &Radians, yaw: &Radians, roll: &Radians) -> Matrix3x3 {
    let rx = Matrix3x3::rotation_x(&pitch);
    let ry = Matrix3x3::rotation_y(&yaw);
    let rz = Matrix3x3::rotation_z(&roll);
    multiply(&multiply(&rz, &ry), &rx)
}

///
/// rotation functions
///
impl Matrix3x3 {
    // todo: rename this to specify column_major
    pub fn rotation_x(radians: &Radians) -> Matrix3x3 {
        let (sin,cos) = radians.radians.sin_cos();
        Matrix3x3 {
            r1c1: 1.0, r1c2: 0.0, r1c3: 0.0,
            r2c1: 0.0, r2c2: cos, r2c3: -sin,
            r3c1: 0.0, r3c2: sin, r3c3: cos,
        }
    }

    // todo: rename this to specify column_major
    pub fn rotation_y(radians: &Radians) -> Matrix3x3 {
        let (sin,cos) = radians.radians.sin_cos();
        Matrix3x3 {
            r1c1: cos, r1c2: 0.0, r1c3: sin,
            r2c1: 0.0, r2c2: 1.0, r2c3: 0.0,
            r3c1: -sin, r3c2: 0.0, r3c3: cos,
        }
    }

    // todo: rename this to specify column_major
    pub fn rotation_z(radians: &Radians) -> Matrix3x3 {
        let (sin,cos) = radians.radians.sin_cos();
        Matrix3x3 {
            r1c1: cos, r1c2: -sin, r1c3: 0.0,
            r2c1: sin, r2c2: cos, r2c3: 0.0,
            r3c1: 0.0, r3c2: 0.0, r3c3: 1.0,
        }
    }
}
