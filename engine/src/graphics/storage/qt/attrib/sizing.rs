use crate::geometry::dim::Dimension2D;

pub enum Sizing {
    Exact { size: f32 },
    Percentage { percent: f32 },
    RemainingSpace {},
}

impl Sizing {
    pub fn from_client_to_dimension(&self, client_dimension: f32) -> f32 {
        match self {
            Sizing::Exact { size } => { *size },
            Sizing::Percentage { percent } => {client_dimension * percent },
            Sizing::RemainingSpace {} => {client_dimension },
        }
    }
}
