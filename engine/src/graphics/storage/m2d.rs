use crate::graphics::image::t2d::Texture2D;
use crate::graphics::geometry::primitive::prim2d::Primitive2D;

pub struct Model2D {
    pub primitives: Vec<Primitive2D>,
    pub textures: Vec<Texture2D>,
}

pub struct Model2DBuilder {
    the_primitives: Vec<Primitive2D>,
    the_textures: Vec<Texture2D>,
}

impl Model2D {
    pub fn new(primitives: Vec<Primitive2D>, textures: Vec<Texture2D>) -> Model2D {
        Model2D {
            primitives,
            textures,
        }
    }
}

impl Model2D {
    pub fn attach_primitive(&mut self, primitive: Primitive2D) {
        self.primitives.push(primitive);
    }

    pub fn attach_texture(&mut self, texture: Texture2D) {
        self.textures.push(texture);
    }
}

impl Model2DBuilder {
    pub fn new() -> Model2DBuilder {
        Model2DBuilder {
            the_primitives: Vec::new(),
            the_textures: Vec::new(),
        }
    }

    pub fn with_primitive(mut self, primitive: Primitive2D) -> Self {
        self.the_primitives.push(primitive);
        self
    }

    pub fn with_texture(mut self, texture: Texture2D) -> Self {
        self.the_textures.push(texture);
        self
    }

    pub fn build(self) -> Model2D {
        Model2D {
            primitives: self.the_primitives,
            textures: self.the_textures,
        }
    }
}
