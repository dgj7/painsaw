use num_traits::Float;
use crate::graphics::image::RawImage;

pub struct Texture2D<F: Float> {
    pub initialized: bool,
    pub id: u32,
    pub image: RawImage,

    pub x: F,
    pub y: F,
    pub scale: F,
    
    pub replacement: Option<RawImage>,
}

pub struct Texture2DBuilder<F: Float> {
    the_image: Option<RawImage>,
    the_x: Option<F>,
    the_y: Option<F>,
    the_scale: Option<F>,
}

impl<F: Float> Texture2D<F> {
    pub fn new(image: RawImage, x: F, y: F, scale: F) -> Texture2D<F> {
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

impl<F: Float> Texture2DBuilder<F> {
    pub fn new() -> Texture2DBuilder<F> {
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

    pub fn with_x(mut self, x: F) -> Self {
        self.the_x = Some(x);
        self
    }

    pub fn with_y(mut self, y: F) -> Self {
        self.the_y = Some(y);
        self
    }

    pub fn with_scale(mut self, scale: F) -> Self {
        self.the_scale = Some(scale);
        self
    }

    pub fn build(self) -> Texture2D<F> {
        Texture2D {
            initialized: false,
            id: 0,
            image: self.the_image.expect("can't create a texture without image data"),
            x: self.the_x.unwrap_or_else(|| F::zero()),
            y: self.the_y.unwrap_or_else(|| F::zero()),
            scale: self.the_scale.unwrap_or_else(|| F::one()),
            replacement: None,
        }
    }
}
