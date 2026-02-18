use crate::geometry::primitive::prim3d::Primitive3D;

pub struct Model3D {
    pub primitives: Vec<Primitive3D>,
}

pub struct Model3DBuilder {
    the_primitives: Vec<Primitive3D>,
}

impl Model3D {
    pub fn new(primitives: Vec<Primitive3D>) -> Model3D {
        Model3D {
            primitives,
        }
    }
}

impl Model3DBuilder {
    pub fn new() -> Model3DBuilder {
        Model3DBuilder {
            the_primitives: Vec::new(),
        }
    }

    pub fn with_primitive(mut self, primitive: Primitive3D) -> Self {
        self.the_primitives.push(primitive);
        self
    }

    pub fn build(self) -> Model3D {
        Model3D {
            primitives: self.the_primitives,
        }
    }
}
