use crate::graphics::geometry::primitive::p3d::Primitive3D;
use num_traits::Float;
use std::ops::{Add, Sub};

pub struct Model3D<F: Float + Add<F> + Sub<F>> {
    pub primitives: Vec<Primitive3D<F>>,
}

pub struct Model3DBuilder<F: Float + Add<F> + Sub<F>> {
    the_primitives: Vec<Primitive3D<F>>,
}

impl<F: Float + Add<F> + Sub<F>> Model3D<F> {
    pub fn new(primitives: Vec<Primitive3D<F>>) -> Model3D<F> {
        Model3D {
            primitives,
        }
    }
}

impl<F: Float + Add<F> + Sub<F>> Model3DBuilder<F> {
    pub fn new() -> Model3DBuilder<F> {
        Model3DBuilder {
            the_primitives: Vec::new(),
        }
    }

    pub fn with_primitive(mut self, primitive: Primitive3D<F>) -> Self {
        self.the_primitives.push(primitive);
        self
    }

    pub fn build(self) -> Model3D<F> {
        Model3D {
            primitives: self.the_primitives,
        }
    }
}
