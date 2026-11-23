use m3d::Model3D;
use std::collections::{BTreeMap, HashMap};
use crate::graphics::storage::o3d::Object3D;
use crate::support::asset::Asset3D;

pub mod m3d;
pub mod id;
pub mod node;
pub mod tree;

// todo: better organization; scene graph?
pub struct Graph3D {
    // todo: potentially remove this? or simplify it to specifically refer to primitives
    models: BTreeMap<String, Model3D>,

    /* reusable assets */
    pub assets: HashMap<usize, Asset3D>,

    /* objects in the world; essentially references to assets */
    pub objects: HashMap<usize, Object3D>,
}

impl Graph3D {
    pub fn new() -> Graph3D {
        Graph3D {
            models: BTreeMap::new(),
            assets: HashMap::new(),
            objects: HashMap::new(),
        }
    }
}

impl Graph3D {
    pub fn attach_asset(&mut self, asset: Asset3D) {
        self.assets.insert(asset.id, asset);
    }

    pub fn attach_object(&mut self, object: Object3D) {
        self.objects.insert(object.id, object);
    }





    pub fn attach(&mut self, name: &str, model: Model3D) {
        self.models.insert(name.to_string(), model);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Model3D)> {
        self.models.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Model3D)> {
        self.models.iter_mut()
    }

    pub fn count(&self) -> usize {
        self.models.len()
    }
}
