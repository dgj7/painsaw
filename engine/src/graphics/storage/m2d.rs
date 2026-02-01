use num_traits::Float;
use std::ops::{Add, Sub};
use crate::graphics::image::t2d::Texture2D;
use crate::graphics::geometry::primitive::prim2d::Primitive2D;

pub struct Model2D<F: Float + Add<F> + Sub<F>> {
    pub primitives: Vec<Primitive2D<F>>,
    pub textures: Vec<Texture2D<F>>,
}

pub struct Model2DBuilder<F: Float + Add<F> + Sub<F>> {
    the_primitives: Vec<Primitive2D<F>>,
    the_textures: Vec<Texture2D<F>>,
}

impl<F: Float + Add<F> + Sub<F>> Model2D<F> {
    pub fn new(primitives: Vec<Primitive2D<F>>, textures: Vec<Texture2D<F>>) -> Model2D<F> {
        Model2D {
            primitives,
            textures,
        }
    }
}

impl<F: Float> Model2D<F> {
    pub fn attach_primitive(&mut self, primitive: Primitive2D<F>) {
        self.primitives.push(primitive);
    }

    pub fn attach_texture(&mut self, texture: Texture2D<F>) {
        self.textures.push(texture);
    }
}

impl<F: Float + Add<F> + Sub<F>> Model2DBuilder<F> {
    pub fn new() -> Model2DBuilder<F> {
        Model2DBuilder {
            the_primitives: Vec::new(),
            the_textures: Vec::new(),
        }
    }

    pub fn with_primitive(mut self, primitive: Primitive2D<F>) -> Self {
        self.the_primitives.push(primitive);
        self
    }

    pub fn with_texture(mut self, texture: Texture2D<F>) -> Self {
        self.the_textures.push(texture);
        self
    }

    pub fn build(self) -> Model2D<F> {
        Model2D {
            primitives: self.the_primitives,
            textures: self.the_textures,
        }
    }
}
