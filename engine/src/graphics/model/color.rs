
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
    
    pub const fn from_rgba_int(red: i32, green: i32, blue: i32, alpha: i32) -> Color {
        Self::from_rgba(
            red as f32 / 255f32,
            green as f32 / 255f32,
            blue as f32 / 255f32,
            alpha as f32 / 255f32,
        )
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

    pub const TRANSPARENT: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.0);
}

#[cfg(test)]
mod test_convert_u8 {
    use crate::graphics::model::color::{Color};

    fn run_case(input: f32, expected: f32) {
        let step1 = Color::from_rgba(input, input, input, input);
        let (red1, green1, blue1, alpha1) = step1.to_u8();

        let step2 = Color::from_rgba_int(red1 as i32, green1 as i32, blue1 as i32, alpha1 as i32);
        validate(expected, &step2);

        let (red2, green2, blue2, alpha2) = step2.to_u8();
        let step3 = Color::from_rgba_int(red2 as i32, green2 as i32, blue2 as i32, alpha2 as i32);

        validate(expected, &step3);
    }

    fn validate(expected: f32, color: &Color) {
        assert_eq!(expected, color.red);
        assert_eq!(expected, color.green);
        assert_eq!(expected, color.blue);
        assert_eq!(expected, color.alpha);
    }

    #[test]
    fn test_min() {
        run_case(0.0, 0.0);
    }

    #[test]
    fn test_max() {
        run_case(1.0, 1.0);
    }
}
