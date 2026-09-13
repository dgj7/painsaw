

pub enum Sizing {
    Exact { size: f32 },
    Percentage { percent: f32 },
    RemainingSpace {},
}

impl Sizing {
    pub fn screen_dimension_to_actual(&self, screen: f32) -> f32 {
        match self {
            Sizing::Exact { size } => *size,
            Sizing::Percentage { percent } => screen * percent,
            Sizing::RemainingSpace {} => screen,
        }
    }
}
