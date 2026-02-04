use crate::graphics::storage::m2d::Model2D;
use num_traits::Float;
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

// todo: better organization; quad tree?
pub struct Graph2D<F: Float + Add<F> + Sub<F>> {
    models: BTreeMap<String, Model2D<F>>
}

impl<F: Float + Add<F> + Sub<F>> Graph2D<F> {
    pub fn new() -> Graph2D<F> {
        Graph2D {
            models: BTreeMap::new(),
        }
    }
}

impl<F: Float> Graph2D<F> {
    pub fn attach(&mut self, name: &str, model: Model2D<F>) {
        self.models.insert(name.to_string(), model);
    }

    pub fn attach_or_update<IF, MF>(&mut self, name: &str, insert: IF, modify: MF)
    where
        IF: Fn() -> Model2D<F>,
        MF: FnOnce(&mut Model2D<F>),
    {
        self.models
            .entry(name.to_string())
            .and_modify(modify)
            .or_insert(insert());
    }
    
    pub fn update<FN>(&mut self, name: &str, fx: FN)
    where
        FN: FnOnce(&mut Model2D<F>),
    {
        self.models
            .entry(name.to_string())
            .and_modify(fx);
    }
}

impl<F: Float> Graph2D<F> {
    pub fn iter(&self) -> impl Iterator<Item=(&String, &Model2D<F>)> {
        self.models.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(&String, &mut Model2D<F>)> {
        self.models.iter_mut()
    }
    
    pub fn count(&self) -> usize {
        self.models.len()
    }
}
