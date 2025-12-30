use std::collections::HashMap;
use std::ops::{Add, Sub};
use num_traits::Float;
use crate::geometry::storage::m3d::Model3D;

// todo: better organization; scene graph?
pub struct Graph3D<F: Float + Add<F> + Sub<F>> {
    pub models: HashMap<String, Model3D<F>>
}

impl<F: Float + Add<F> + Sub<F>> Graph3D<F> {
    pub fn new() -> Graph3D<F> {
        Graph3D { 
            models: HashMap::new(),
        }
    }
}
