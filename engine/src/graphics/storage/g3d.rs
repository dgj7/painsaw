use m3d::Model3D;
use std::collections::BTreeMap;

pub mod m3d;
pub mod id;
pub mod node;
pub mod tree;

// todo: better organization; scene graph?
pub struct Graph3D {
    models: BTreeMap<String, Model3D>,
}

impl Graph3D {
    pub fn new() -> Graph3D {
        Graph3D {
            models: BTreeMap::new(),
        }
    }
}

impl Graph3D {
    pub fn attach(&mut self, name: &str, model: Model3D) {
        self.models.insert(name.to_string(), model);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Model3D)> {
        self.models.iter()
    }
}
