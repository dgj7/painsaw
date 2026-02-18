use crate::geometry::orient::matrix::m4x4::Matrix4x4;
use crate::geometry::primitive::v3d::Vertex3D;

impl Matrix4x4 {
    pub fn from(x_right: Vertex3D, y_up: Vertex3D, z_forward: Vertex3D, position: Vertex3D) -> Matrix4x4 {
        Matrix4x4 {
            c1r1: x_right.x,
            c1r2: x_right.y,
            c1r3: x_right.z,
            c1r4: 0.0,

            c2r1: y_up.x,
            c2r2: y_up.y,
            c2r3: y_up.z,
            c2r4: 0.0,


            c3r1: z_forward.x,
            c3r2: z_forward.y,
            c3r3: z_forward.z,
            c3r4: 0.0,

            c4r1: position.x,
            c4r2: position.y,
            c4r3: position.z,
            c4r4: 0.0,// todo: i think this should be 1.0
        }
    }
    
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            c1r1: 1.0,
            c1r2: 0.0,
            c1r3: 0.0,
            c1r4: 0.0,

            c2r1: 0.0,
            c2r2: 1.0,
            c2r3: 0.0,
            c2r4: 0.0,


            c3r1: 0.0,
            c3r2: 0.0,
            c3r3: 1.0,
            c3r4: 0.0,

            c4r1: 0.0,
            c4r2: 0.0,
            c4r3: 0.0,
            c4r4: 1.0,
        }
    }
}
