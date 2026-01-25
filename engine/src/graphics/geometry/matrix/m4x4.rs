use crate::graphics::geometry::point::p3d::Point3D;
use num_traits::Float;

///
/// store a 4x4 matrix, containing rotation, translation and scaling of an object or camera.
/// a matrix could be interpreted as column-major or row-major.
///
/// column-major(opengl, unity): each column represents 3 (basis) vectors and position:
/// 1) c1: x-axis (right)
/// 2) c2: y-axis (up)
/// 3) c3: z-axis (forward)
/// 4) c4: translation (position)
///
/// the _scale_ is stored as the _length_ of each basis vector:
/// 1) x-scale is equal to the length of the x/right vector
/// 2) y-scale is equal to the length of the y/up vector
/// 3) z-scale is equal to the length of the z/forward vector
///
/// visually, the column-major 4x4 matrix is organized as such:
/// right-x  up-x  forward-x  position-x
/// right-y  up-y  forward-y  position-y
/// right-z  up-z  forward-z  position-z
/// 0        0     0          1
///
/// row-major (directx/unreal): each row represents 3 (basis) vectors and position.
///
pub struct Matrix4x4<F: Float> {
    /* column1: x(right) */
    pub c1r1: F,
    pub c1r2: F,
    pub c1r3: F,
    pub c1r4: F,

    /* column2: y(up) */
    pub c2r1: F,
    pub c2r2: F,
    pub c2r3: F,
    pub c2r4: F,

    /* column3: z(forward) */
    pub c3r1: F,
    pub c3r2: F,
    pub c3r3: F,
    pub c3r4: F,

    /* column4: translation(position) */
    pub c4r1: F,
    pub c4r2: F,
    pub c4r3: F,
    pub c4r4: F,
}

impl<F: Float> Matrix4x4<F> {
    pub fn from(x_right: Point3D<F>, y_up: Point3D<F>, z_forward: Point3D<F>, position: Point3D<F>) -> Matrix4x4<F> {
        Matrix4x4 {
            c1r1: x_right.x,
            c1r2: x_right.y,
            c1r3: x_right.z,
            c1r4: F::from(0.0).unwrap(),

            c2r1: y_up.x,
            c2r2: y_up.y,
            c2r3: y_up.z,
            c2r4: F::from(0.0).unwrap(),


            c3r1: z_forward.x,
            c3r2: z_forward.y,
            c3r3: z_forward.z,
            c3r4: F::from(0.0).unwrap(),

            c4r1: position.x,
            c4r2: position.y,
            c4r3: position.z,
            c4r4: F::from(0.0).unwrap(),
        }
    }
}

impl<F: Float> Matrix4x4<F> {
    pub fn column_major_x_right(&self) -> Point3D<F> {
        Point3D {
            x: self.c1r1,
            y: self.c1r2,
            z: self.c1r3,
        }
    }

    pub fn column_major_y_up(&self) -> Point3D<F> {
        Point3D {
            x: self.c2r1,
            y: self.c2r2,
            z: self.c2r3,
        }
    }

    pub fn column_major_z_forward(&self) -> Point3D<F> {
        Point3D {
            x: self.c3r1,
            y: self.c3r2,
            z: self.c3r3,
        }
    }

    pub fn column_major_translation(&self) -> Point3D<F> {
        Point3D {
            x: self.c4r1,
            y: self.c4r2,
            z: self.c4r3,
        }
    }

    pub fn column_major_x_scale(&self) -> F {
        scale(self.column_major_x_right())
    }

    pub fn column_major_y_scale(&self) -> F {
        scale(self.column_major_y_up())
    }

    pub fn column_major_z_scale(&self) -> F {
        scale(self.column_major_z_forward())
    }
}

impl<F: Float> Default for Matrix4x4<F> {
    fn default() -> Matrix4x4<F> {
        Matrix4x4::from(Point3D::create_x_unit(), Point3D::create_y_unit(), Point3D::create_z_unit(), Point3D::origin())
    }
}

///
/// calculate the scale for the given axis.
///
/// presumes the axis is not normalized.
///
fn scale<F: Float>(axis: Point3D<F>) -> F {
    ((axis.x * axis.x) + (axis.y * axis.y) + (axis.z * axis.z)).sqrt()
}
