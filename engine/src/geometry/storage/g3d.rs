use crate::geometry::storage::m3d::Model3D;
use num_traits::Float;
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

// todo: better organization; scene graph?
pub struct Graph3D<F: Float + Add<F> + Sub<F>> {
    pub models: BTreeMap<String, Model3D<F>>
}

impl<F: Float + Add<F> + Sub<F>> Graph3D<F> {
    pub fn new() -> Graph3D<F> {
        Graph3D { 
            models: BTreeMap::new(),
        }
    }
}
