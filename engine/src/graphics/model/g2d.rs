use crate::graphics::model::m2d::Model2D;
use num_traits::Float;
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

// todo: better organization; quad tree?
pub struct Graph2D<F: Float + Add<F> + Sub<F>> {
    pub models: BTreeMap<String, Model2D<F>>
}

impl<F: Float + Add<F> + Sub<F>> Graph2D<F> {
    pub fn new() -> Graph2D<F> {
        Graph2D {
            models: BTreeMap::new(),
        }
    }
}
