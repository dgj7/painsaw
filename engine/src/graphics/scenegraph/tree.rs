use crate::graphics::scenegraph::id::NodeId;
use crate::graphics::scenegraph::node::SceneNode;
use std::collections::HashMap;

///
/// Main scene graph data structure.
///
/// nodes are not directly attached, they are referenced by unique identifiers.
///
#[allow(unused)] // todo: remove
pub struct SceneGraph {
    nodes: HashMap<NodeId, SceneNode>,
    root: NodeId,
}
