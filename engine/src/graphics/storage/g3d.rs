use crate::graphics::storage::m3d::Model3D;
use num_traits::Float;
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

// todo: better organization; scene graph?
pub struct Graph3D<F: Float + Add<F> + Sub<F>> {
    models: BTreeMap<String, Model3D<F>>,
}

impl<F: Float + Add<F> + Sub<F>> Graph3D<F> {
    pub fn new() -> Graph3D<F> {
        Graph3D { 
            models: BTreeMap::new(),
        }
    }
}

impl<F: Float + Add<F> + Sub<F> + Add<F>> Graph3D<F> {
    pub fn attach(&mut self, name: &str, model: Model3D<F>) {
        self.models.insert(name.to_string(), model);
    }

    pub fn iter(&self) -> impl Iterator<Item=(&String, &Model3D<F>)> {
        self.models.iter()
    }
}
