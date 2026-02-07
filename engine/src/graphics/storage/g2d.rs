use crate::graphics::storage::m2d::Model2D;
use std::collections::BTreeMap;

// todo: better organization; quad tree?
pub struct Graph2D {
    models: BTreeMap<String, Model2D>
}

impl Graph2D {
    pub fn new() -> Graph2D {
        Graph2D {
            models: BTreeMap::new(),
        }
    }
}

impl Graph2D {
    pub fn attach(&mut self, name: &str, model: Model2D) {
        self.models.insert(name.to_string(), model);
    }

    pub fn attach_or_update<IF, MF>(&mut self, name: &str, insert: IF, modify: MF)
    where
        IF: Fn() -> Model2D,
        MF: FnOnce(&mut Model2D),
    {
        self.models
            .entry(name.to_string())
            .and_modify(modify)
            .or_insert(insert());
    }
    
    pub fn update<FN>(&mut self, name: &str, fx: FN)
    where
        FN: FnOnce(&mut Model2D),
    {
        self.models
            .entry(name.to_string())
            .and_modify(fx);
    }
}

impl Graph2D {
    pub fn iter(&self) -> impl Iterator<Item=(&String, &Model2D)> {
        self.models.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(&String, &mut Model2D)> {
        self.models.iter_mut()
    }
    
    pub fn count(&self) -> usize {
        self.models.len()
    }
}
