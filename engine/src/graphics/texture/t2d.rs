use crate::geometry::primitive::v3d::Vertex3D;
use crate::support::image::RawImage;

pub struct Texture {
    pub initialized: bool,
    pub id: u32,
    pub image: RawImage,

    pub location: Vertex3D,
    pub scale: f32,

    pub replacement: Option<RawImage>,
}

pub struct Texture2DBuilder {
    the_image: Option<RawImage>,
    the_location: Option<Vertex3D>,
    the_scale: Option<f32>,
}

impl Texture {
    pub fn new_2d(image: RawImage, x: f32, y: f32, scale: f32) -> Texture {
        Texture::new_3d(image, x, y, 0.0, scale)
    }
    
    pub fn new_3d(image: RawImage, x: f32, y: f32, z: f32, scale: f32) -> Texture {
        Texture {
            initialized: false,
            id: 0,
            image,

            location: Vertex3D { x, y, z },

            scale,

            replacement: None,
        }
    }
}

impl Texture2DBuilder {
    pub fn new() -> Texture2DBuilder {
        Texture2DBuilder {
            the_image: None,
            the_location: None,
            the_scale: None,
        }
    }

    pub fn with_image(mut self, image: RawImage) -> Self {
        self.the_image = Some(image);
        self
    }

    pub fn with_location(mut self, corner: Vertex3D) -> Self {
        self.the_location = Some(corner);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.the_scale = Some(scale);
        self
    }

    pub fn build(self) -> Texture {
        Texture {
            initialized: false,
            id: 0,
            image: self.the_image.expect("can't create a texture without texture data"),
            location: self.the_location.expect("can't create a texture without specifying it's location"),
            scale: self.the_scale.unwrap_or_else(|| 1.0),
            replacement: None,
        }
    }
}
