use std::collections::HashMap;
use std::ops::{Add, Sub};
use num_traits::Float;
use crate::geometry::storage::m2d::Model2D;

// todo: better organization; quad tree?
pub struct Graph2D<F: Float + Add<F> + Sub<F>> {
    pub models: HashMap<String, Model2D<F>>
}

impl<F: Float + Add<F> + Sub<F>> Graph2D<F> {
    pub fn new() -> Graph2D<F> {
        Graph2D {
            models: HashMap::new(),
        }
    }
}
