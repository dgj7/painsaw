use crate::support::image::RawImage;

pub struct Texture2D {
    pub initialized: bool,
    pub id: u32,
    pub image: RawImage,

    pub x: f32,
    pub y: f32,
    pub scale: f32,

    pub replacement: Option<RawImage>,
}

pub struct Texture2DBuilder {
    the_image: Option<RawImage>,
    the_x: Option<f32>,
    the_y: Option<f32>,
    the_scale: Option<f32>,
}

impl Texture2D {
    pub fn new(image: RawImage, x: f32, y: f32, scale: f32) -> Texture2D {
        Texture2D {
            initialized: false,
            id: 0,
            image,

            x,
            y,
            scale,

            replacement: None,
        }
    }
}

impl Texture2DBuilder {
    pub fn new() -> Texture2DBuilder {
        Texture2DBuilder {
            the_image: None,
            the_x: None,
            the_y: None,
            the_scale: None,
        }
    }

    pub fn with_image(mut self, image: RawImage) -> Self {
        self.the_image = Some(image);
        self
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.the_x = Some(x);
        self
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.the_y = Some(y);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.the_scale = Some(scale);
        self
    }

    pub fn build(self) -> Texture2D {
        Texture2D {
            initialized: false,
            id: 0,
            image: self.the_image.expect("can't create a texture without texture data"),
            x: self.the_x.unwrap_or_else(|| 0.0),
            y: self.the_y.unwrap_or_else(|| 0.0),
            scale: self.the_scale.unwrap_or_else(|| 1.0),
            replacement: None,
        }
    }
}
