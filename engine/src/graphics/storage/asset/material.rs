use crate::graphics::color::Color;
use crate::graphics::texture::Texture;

pub struct Material {
    pub name: Option<String>,
    pub diffuse_texture: Option<Texture>,
    pub diffuse_color: Color,
    
    // todo: for later
    pub normal_texture: Option<Texture>,
    pub specular_texture: Option<Texture>,
    pub specular_color: Color,
    pub shininess: f32,
}
