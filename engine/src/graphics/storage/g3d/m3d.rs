use crate::geometry::primitive::prim3d::Primitive3D;
use crate::graphics::texture::Texture;

pub struct Model3D {
    pub primitives: Vec<Primitive3D>,
    pub textures: Vec<Texture>,

    pub visible: bool,
}

pub struct Model3DBuilder {
    the_primitives: Vec<Primitive3D>,
    the_textures: Vec<Texture>,
    the_visible: Option<bool>,
}

impl Model3D {
    pub fn new(primitives: Vec<Primitive3D>, textures: Vec<Texture>, visible: bool) -> Model3D {
        Model3D {
            primitives,
            textures,
            visible,
        }
    }
}

impl Model3DBuilder {
    pub fn new() -> Model3DBuilder {
        Model3DBuilder { 
            the_primitives: Vec::new(),
            the_textures: Vec::new(),
            the_visible: None,
        }
    }

    pub fn with_primitive(mut self, primitive: Primitive3D) -> Self {
        self.the_primitives.push(primitive);
        self
    }
    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.the_textures.push(texture);
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.the_visible = Some(visible);
        self
    }

    pub fn build(self) -> Model3D {
        Model3D { 
            primitives: self.the_primitives,
            textures: self.the_textures,
            visible: self.the_visible.unwrap_or_else(|| true),
        }
    }
}
