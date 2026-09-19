use crate::geometry::dim::Dimension2D;

pub enum Sizing {
    Exact { size: f32 },
    Percentage { percent: f32 },
    RemainingSpace {},
}

impl Sizing {
    pub fn container_to_dimension(&self, container_dimension: f32) -> f32 {
        match self {
            Sizing::Exact { size } => { *size },
            Sizing::Percentage { percent } => { container_dimension * percent },
            Sizing::RemainingSpace {} => { container_dimension },
        }
    }
}
