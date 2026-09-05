use crate::geometry::primitive::prim2d::Primitive2D;
use crate::graphics::texture::t2d::Texture2D;

pub struct Model2D {
    pub primitives: Vec<Primitive2D>,
    pub textures: Vec<Texture2D>,

    pub visible: bool,
}

pub struct Model2DBuilder {
    the_primitives: Vec<Primitive2D>,
    the_textures: Vec<Texture2D>,
    the_visible: Option<bool>,
}

impl Model2D {
    pub fn new(primitives: Vec<Primitive2D>, textures: Vec<Texture2D>, visible: bool) -> Model2D {
        Model2D {
            primitives,
            textures,
            visible,
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
            the_visible: None,
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

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.the_visible = Some(visible);
        self
    }

    pub fn build(self) -> Model2D {
        Model2D {
            primitives: self.the_primitives,
            textures: self.the_textures,
            visible: self.the_visible.unwrap_or_else(|| true),
        }
    }
}
