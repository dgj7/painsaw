
#[derive(Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

// todo: should these float values be clamped?  what to do if they're above an expected value?  panic??

impl Color {
    pub const fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {red, green, blue, alpha }
    }
    
    pub const fn from_rgb(red: f32, green: f32, blue: f32) -> Color {
        Self::from_rgba(red, green, blue, 1.0)
    }
    
    pub fn to_u8(&self) -> (u8, u8, u8, u8) {
        let red = (self.red * 255.0) as u8;
        let green = (self.green * 255.0) as u8;
        let blue = (self.blue * 255.0) as u8;
        let alpha = (self.alpha * 255.0) as u8;
        (red, green, blue, alpha)
    }
    
    pub const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::from_rgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::from_rgb(0.0, 0.0, 1.0);
    
    pub const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0);

    pub const TRANSPARENT: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.0);
}
