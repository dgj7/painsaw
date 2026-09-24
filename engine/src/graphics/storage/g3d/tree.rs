use crate::graphics::storage::g3d::id::NodeId;
use crate::graphics::storage::g3d::node::SceneNode;
use std::collections::HashMap;

///
/// Main scene graph data structure.
///
/// nodes are not directly attached, they are referenced by unique identifiers.
///\
pub struct SceneGraph {
    nodes: HashMap<NodeId, SceneNode>,
    root: NodeId,
}
