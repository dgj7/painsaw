use crate::geometry::orient::Orientation;
use crate::graphics::scenegraph::id::NodeId;

pub struct SceneNode {
    /* attachments */
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,

    /* orientation of the node */
    pub orientation: Orientation,

    /* models attached here */
    // todo: model(s) (Model3D)
}
