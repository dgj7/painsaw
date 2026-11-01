//!
//! storage for an instance of an object in the game world.
//! 
//! an object usually references an asset (information about what to render), as well as some additional info about what it is.
//!

use crate::geometry::orient::Orientation;

///
/// an object instance in 3d space.
/// 
pub struct Object3D {
    /* what is the object */
    pub id: usize,
    pub name: Option<String>,
    
    /* the position and orientation of the object */
    pub transform: Orientation,
    
    /* what to render */
    pub asset_id: usize,

    /* object traits */
    pub visible: bool,
    pub clipping: bool,
}
