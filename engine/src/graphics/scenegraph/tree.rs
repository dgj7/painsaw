use std::collections::HashMap;
use crate::graphics::scenegraph::id::NodeId;
use crate::graphics::scenegraph::node::SceneNode;

///
/// Main scene graph data structure.
///
/// nodes are not directly attached, they are referenced by unique identifiers.
///
pub struct SceneGraph {
    nodes: HashMap<NodeId, SceneNode>,
    root: NodeId,
}
