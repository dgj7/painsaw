#[derive(Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub const fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {red, green, blue, alpha }
    }
    
    pub const fn from_rgb(red: f32, green: f32, blue: f32) -> Color {
        Color {red, green, blue, alpha: 1.0}
    }
    
    pub const fn from_rgba_int(red: i32, green: i32, blue: i32, alpha: i32) -> Color {
        Color { red: (red / 256) as f32, green: (green / 256) as f32, blue: (blue / 256) as f32, alpha: alpha as f32 }
    }
    
    pub const fn from_rgb_int(red: i32, green: i32, blue: i32) -> Color {
        Color { red: (red / 256) as f32, green: (green / 256) as f32, blue: (blue / 256) as f32, alpha: 1.0 }
    }
    
    pub const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::from_rgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::from_rgb(0.0, 0.0, 1.0);
    
    pub const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0);
}
