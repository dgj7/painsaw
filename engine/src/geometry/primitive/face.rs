use glcore::{GL_BACK, GL_FRONT, GL_FRONT_AND_BACK};

pub enum PolygonFace {
    Front,
    Back,
    FrontAndBack,
}

impl PolygonFace {
    pub fn to_u32(&self) -> u32 {
        match self {
            PolygonFace::Back => GL_BACK,
            PolygonFace::Front => GL_FRONT,
            PolygonFace::FrontAndBack => GL_FRONT_AND_BACK,
        }
    }
}
