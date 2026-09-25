use crate::geometry::orient::matrix::m4x4::Matrix4x4;
use crate::geometry::primitive::v3d::Vertex3D;

impl Matrix4x4 {
    pub fn from(
        x_right: Vertex3D,
        y_up: Vertex3D,
        z_forward: Vertex3D,
        position: Vertex3D,
    ) -> Matrix4x4 {
        Matrix4x4 {
            r1c1: x_right.x,
            r2c1: x_right.y,
            r3c1: x_right.z,
            r4c1: 0.0,

            r1c2: y_up.x,
            r2c2: y_up.y,
            r3c2: y_up.z,
            r4c2: 0.0,

            r1c3: z_forward.x,
            r2c3: z_forward.y,
            r3c3: z_forward.z,
            r4c3: 0.0,

            r1c4: position.x,
            r2c4: position.y,
            r3c4: position.z,
            r4c4: 0.0, // todo: i think this should be 1.0
        }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            r1c1: 1.0, r1c2: 0.0, r1c3: 0.0, r1c4: 0.0,
            r2c1: 0.0, r2c2: 1.0, r2c3: 0.0, r2c4: 0.0,
            r3c1: 0.0, r3c2: 0.0, r3c3: 1.0, r3c4: 0.0,
            r4c1: 0.0, r4c2: 0.0, r4c3: 0.0, r4c4: 1.0,
        }
    }
}
